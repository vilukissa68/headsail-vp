#!/usr/bin/env python3

import os
from pathlib import Path
import pandas as pd
from PIL import Image
import numpy as np
import librosa as lb
import re
import serial
from sklearn.metrics import accuracy_score, confusion_matrix, classification_report
import tensorflow as tf



UART = '/tmp/uart0'
ROOT_PATH = Path(__file__).parents[0]
DATA_DIR = ROOT_PATH / "dev_data"
KWS_DATA_DIR = DATA_DIR / "kws01"
VWW_DATA_DIR = DATA_DIR / "vw_coco2014_96"
VWW_NON_PERSON_DATA_DIR =  VWW_DATA_DIR / "non_person"
VWW_PERSON_DATA_DIR = VWW_DATA_DIR / "person"
IC_DATA_DIR = DATA_DIR / "cifar-10-batches-py"
AD_DATA_DIR = DATA_DIR / "ToyCar" / "test"

def accuracy_report(gt, prediction):
    print("Accuracy: {:.3f}".format(accuracy_score(gt, prediction)))
    print("Confusion matrix:\n{}".format(confusion_matrix(gt, prediction)))
    print(classification_report(gt, prediction))

def send_stimulus(data):
    print("Writing {} bytes as stimulus...".format(len(data)))
    ser = serial.Serial(UART, 9600)
    ser.write(data)
    ser.write(b'\n') # EOL signifies end of stimulus
    ser.close()

def wait_for_result():
    return 1
    ser = serial.Serial(UART,  9600)
    output = ser.readline()
    ser.close()
    return output

def read_kws_file(path):
    with open(path, mode="rb") as file:
        content = file.read()
    return content

def run_kws():
    df = pd.read_csv(KWS_DATA_DIR / "y_labels.csv", names=["filename", "no_classes", "class"])
    print("Input shape: ( ,{})".format(len(read_kws_file(KWS_DATA_DIR / df["filename"][0]))))

    predictions = []
    for (i, filename) in enumerate(df["filename"]):
        data = read_kws_file(KWS_DATA_DIR / filename)
        send_stimulus(data)

        predictions.append(wait_for_result())

    accuracy_report(df["class"], predictions)

def get_kws_stimulus():
    df = pd.read_csv(KWS_DATA_DIR / "y_labels.csv", names=["filename", "no_classes", "class"])
    data = read_kws_file(KWS_DATA_DIR / df["filename"][0])
    print("Expected label:", df["class"][0])
    return data

def read_vww_file(path):
    #Image loading and preprocessing
    image = tf.io.read_file(str(path))
    image = tf.image.decode_jpeg(image, channels=3)
    image = tf.image.resize(image, [96,96])
    image = tf.cast(image, tf.uint8)
    byte_array = image.numpy()
    return byte_array

def run_vww():
    items = os.listdir(VWW_NON_PERSON_DATA_DIR)
    non_persons = [item for item in items if os.path.isfile(os.path.join(VWW_NON_PERSON_DATA_DIR, item)) and item.startswith("COCO_val")]

    items = os.listdir(VWW_PERSON_DATA_DIR)
    persons = [item for item in items if os.path.isfile(os.path.join(VWW_PERSON_DATA_DIR, item)) and item.startswith("COCO_val")]

    print("Number of non_persons", len(non_persons))
    print("Number of persons", len(persons))
    print("Input shape: ", np.shape(read_vww_file(VWW_NON_PERSON_DATA_DIR / non_persons[0])))

    # Generate ground truth array
    gt_non = [0 for _ in non_persons]
    gt_yes = [1 for _ in persons]
    gt = gt_non + gt_yes

    predictions = []
    for x in non_persons:
        data = read_vww_file(VWW_NON_PERSON_DATA_DIR / x)

        send_stimulus(data.tobytes())
        predictions.append(wait_for_result())

    for x in persons:
        data = read_vww_file(VWW_PERSON_DATA_DIR / x)
        send_stimulus(data.tobytes())
        predictions.append(wait_for_result())

    accuracy_report(gt, predictions)

def get_vww_stimulus():
    # items = os.listdir(VWW_PERSON_DATA_DIR)
    # persons = [item for item in items if os.path.isfile(os.path.join(VWW_PERSON_DATA_DIR, item)) and item.startswith("COCO_val")]
    # data = read_vww_file(VWW_PERSON_DATA_DIR / persons[1])

    items = os.listdir(VWW_NON_PERSON_DATA_DIR)
    non_persons = [item for item in items if os.path.isfile(os.path.join(VWW_NON_PERSON_DATA_DIR, item)) and item.startswith("COCO_val")]
    data = read_vww_file(VWW_NON_PERSON_DATA_DIR / non_persons[0])
    print("Expected label: 1")
    return data.tobytes()


def run_ic():
    import pickle
    with open(IC_DATA_DIR / "test_batch", "rb") as file:
        data = pickle.load(file, encoding='bytes')
    print("Input shape: {}".format(np.shape(data[b'data'][0])))

    predictions = []
    for image in data[b'data']:
        send_stimulus(image.tobytes())

        # Wait for inference result
        predictions.append(wait_for_result())
        pass

    # Evaluate predictions
    accuracy_report(data[b'labels'], predictions)

def get_ic_stimulus():
    import pickle
    with open(IC_DATA_DIR / "test_batch", "rb") as file:
        data = pickle.load(file, encoding='bytes')
    print("Expected label:", data[b'labels'][0])
    return data[b'data'][0].tobytes()


def main():
    #run_kws()
    #run_vww()
    run_ic()

if __name__ == "__main__":
    main()
