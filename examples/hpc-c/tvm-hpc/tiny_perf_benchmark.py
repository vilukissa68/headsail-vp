#!/usr/bin/env python3

import argparse
import os
from pathlib import Path
import pandas as pd
from PIL import Image
import numpy as np
import librosa as lb
import re
import serial
from sklearn.metrics import accuracy_score, confusion_matrix, classification_report
from matplotlib import pyplot as plt
import tensorflow as tf
import time
import numpy as np

UART = '/tmp/uart0'
ROOT_PATH = Path(__file__).parents[0]
DATA_DIR = ROOT_PATH / "dev_data"
KWS_DATA_DIR = DATA_DIR / "kws01"
VWW_DATA_DIR = DATA_DIR / "vw_coco2014_96"
VWW_NON_PERSON_DATA_DIR =  VWW_DATA_DIR / "non_person"
VWW_PERSON_DATA_DIR = VWW_DATA_DIR / "person"
IC_DATA_DIR = DATA_DIR / "cifar-10-batches-py"
AD_DATA_DIR = DATA_DIR / "ToyCar" / "test"


# UTILS
def print_matrix(arr, format_type='signed'):
    for row in arr:
        for elem in row:
            if format_type == 'signed':
                print(f"{int(elem):d}", end="\t")  # Signed decimal
            elif format_type == 'unsigned':
                print(f"{int(elem) & 0xFFFFFFFF:d}", end="\t")  # Unsigned decimal
            elif format_type == 'hex':
                print(f"{int(elem) & 0xFFFFFFFF:08x}", end="\t")  # Hexadecimal
            else:
                raise ValueError("Invalid format_type. Use 'signed', 'unsigned', or 'hex'.")
        print()

def accuracy_report(gt, prediction):
    print("Accuracy: {:.3f}".format(accuracy_score(gt, prediction)))
    print("Confusion matrix:\n{}".format(confusion_matrix(gt, prediction)))
    print(classification_report(gt, prediction))

def send_stimulus(data, label=None):
    print("Writing {} bytes as stimulus...".format(len(data)))
    if label is not None:
        print("Expected label: {}".format(label))
    ser = serial.Serial(UART, 9600)
    print("Sending stimulus...")
    ser.write(bytes(data))
    ser.close()

def wait_for_result():
    print("Waiting for results...")
    ser = serial.Serial(UART,  9600)
    output = ser.readline()
    while output != b'Prediction:\n':
        output = ser.readline()
    output = ser.readline()
    ser.close()
    output = bytearray(output)
    results = []
    for x in output:
        results.append(((x & 0xff) ^ 0x80) - 0x80) # Append signed
    results = results[:-1] # Remove line break
    print(results)
    print("Predicted class: {}".format(np.argmax(results)))
    print("\n")
    return results


# KWS
def read_kws_file(path):
    with open(path, mode="rb") as file:
        content = file.read()
    return content

def get_kws_stimulus():
    df = pd.read_csv(KWS_DATA_DIR / "y_labels.csv", names=["filename", "no_classes", "class"])
    data = read_kws_file(KWS_DATA_DIR / df["filename"][0])
    print("Expected label:", df["class"][0])
    return data

def run_kws(total_samples=200):
    df = pd.read_csv(KWS_DATA_DIR / "y_labels.csv", names=["filename", "no_classes", "class"])

    class_counts = df["class"].value_counts()

    print(f"Classes in dataset: {class_counts}")

    num_classes = len(class_counts)

    # Calculate the number of samples per class, and how many extra samples to distribute
    base_samples_per_class = total_samples // num_classes
    extra_samples = total_samples % num_classes

    balanced_df = pd.DataFrame()
    for class_id in class_counts.index:
        class_samples = df[df["class"] == class_id].sample(base_samples_per_class)
        balanced_df = pd.concat([balanced_df, class_samples])

    remaining_samples_needed = extra_samples
    for class_id in class_counts.index:
        if remaining_samples_needed == 0:
            break
        # If there are more remaining samples, sample one more from this class
        class_samples = df[df["class"] == class_id].sample(1)
        balanced_df = pd.concat([balanced_df, class_samples])
        remaining_samples_needed -= 1

    # Shuffle the resulting balanced dataset
    balanced_df = balanced_df.sample(frac=1, random_state=42)

    predictions = []
    for (i, filename) in enumerate(balanced_df["filename"]):
        data = read_kws_file(KWS_DATA_DIR / filename)
        send_stimulus(data, df["class"][i])
        predictions.append(np.argmax(wait_for_result()))

        # Mid run report
        accuracy_report(balanced_df["class"][:len(predictions)+1], predictions)

    print("Final accuracy report for Keyword Spotting:")
    accuracy_report(balanced_df["class"], predictions)


# VWW
def read_vww_file(path):
    #Image loading and preprocessing
    image = tf.io.read_file(str(path))
    image = tf.image.decode_jpeg(image, channels=3)
    image = tf.image.resize(image, [96,96])
    image = tf.cast(image, tf.uint8)
    byte_array = image.numpy()
    return byte_array

def get_vww_stimulus():
    items = os.listdir(VWW_NON_PERSON_DATA_DIR)
    non_persons = [item for item in items if os.path.isfile(os.path.join(VWW_NON_PERSON_DATA_DIR, item)) and item.startswith("COCO_val")]
    data = read_vww_file(VWW_NON_PERSON_DATA_DIR / non_persons[0])
    print("Expected label: 1")
    return data.tobytes()

def run_vww(total_samples=100):
    items = os.listdir(VWW_NON_PERSON_DATA_DIR)
    non_persons = [item for item in items if os.path.isfile(os.path.join(VWW_NON_PERSON_DATA_DIR, item)) and item.startswith("COCO_val")]

    items = os.listdir(VWW_PERSON_DATA_DIR)
    persons = [item for item in items if os.path.isfile(os.path.join(VWW_PERSON_DATA_DIR, item)) and item.startswith("COCO_val")]

    print("Number of non_persons", len(non_persons))
    print("Number of persons", len(persons))
    print("Input shape: ", np.shape(read_vww_file(VWW_NON_PERSON_DATA_DIR / non_persons[0])))

    # Calculate balanced number of samples for each category
    samples_per_class = min(len(non_persons), len(persons), total_samples // 2)

    # Select samples for each category
    selected_non_persons = non_persons[:samples_per_class]
    selected_persons = persons[:samples_per_class]

    # Generate ground truth array
    gt = []

    predictions = []
    for non_person, person in zip(selected_non_persons, selected_persons):
        # Non person sample
        data_non = read_vww_file(VWW_NON_PERSON_DATA_DIR / non_person)
        send_stimulus(data_non.tobytes())
        predictions.append(wait_for_result())
        gt.append(0)

        # Person sample
        data_person = read_vww_file(VWW_PERSON_DATA_DIR / person)
        send_stimulus(data_person.tobytes())
        predictions.append(wait_for_result())
        gt.append(1)

        # Mid-run report
        accuracy_report(gt, predictions)

    print("Final accuracy report for Visual Wakeup Word:")
    accuracy_report(gt, predictions)

# IC
def get_ic_stimulus():
    import pickle
    with open(IC_DATA_DIR / "test_batch", "rb") as file:
        data = pickle.load(file, encoding='bytes')
    print("Expected label:", data[b'labels'][0])
    return data[b'data'][0].tobytes()

def run_ic(total_samples=200):
    import pickle
    with open(IC_DATA_DIR / "test_batch", "rb") as file:
        data = pickle.load(file, encoding='bytes')
    print("Input shape: {}".format(np.shape(data[b'data'][0])))

    images = data[b'data']
    labels = data[b'labels']

    class_samples = {i: [] for i in range(10)}
    samples_per_class = total_samples // 10

    # Find samples
    for img, label in zip(images, labels):
        if len(class_samples[label]) < samples_per_class:
            class_samples[label].append(img)
        if all(len(class_samples[c]) == samples_per_class for c in class_samples):
            break

    selected_images = []
    selected_labels = []
    for label, samples in class_samples.items():
        selected_images.extend(samples)
        selected_labels.extend([label] * len(samples))

    # Convert to numpy
    selected_images = np.array(selected_images)
    selected_labels = np.array(selected_labels)

    predictions = []

    # Run inference on samples
    for (i, image) in enumerate(selected_images):
        #FROM CHW to HWC
        print("Running inference for image {}/{}".format(i, len(data[b'data'])))
        image = np.reshape(image, (3, 32, 32))
        image = np.rollaxis(image, 0, 3)
        image = image - 128
        image = np.reshape(image, (3072))
        label = selected_labels[i]
        send_stimulus(image.tobytes(), label)

        # Wait for inference result
        prediction = (np.argmax(wait_for_result()))
        predictions.append(prediction)

        # Mid-run report
        accuracy_report(selected_labels[:len(predictions)+1], predictions)

    print("Final accuracy report for Image Classification:")
    accuracy_report(selected_labels, predictions)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-b", "--benchmark", required=True)
    parser.add_argument("-s", "--samples", required=False, type=int)
    opts = parser.parse_args()
    if opts.benchmark == "kws":
        if opts.samples:
            run_kws(opts.samples)
        else:
            run_kws()
    elif opts.benchmark == "vww":
        if opts.samples:
            run_vww(opts.samples)
        else:
            run_vww()
    elif opts.benchmark == "ic":
        if opts.samples:
            run_ic(opts.samples)
        else:
            run_ic()
    else:
        print("Bad benchmark! Available benchmarks are: kws, vww, ic")

if __name__ == "__main__":
    main()
