#!/usr/bin/env python3

import os
from pathlib import Path
import pandas as pd
from PIL import Image
import io


ROOT_PATH = Path(__file__).parents[0]
DATA_DIR = ROOT_PATH / "dev_data"
KWS_DATA_DIR = DATA_DIR / "kws01"
VWW_DATA_DIR = DATA_DIR / "vw_coco2014_96"
VWW_NON_PERSON_DATA_DIR =  VWW_DATA_DIR / "non_person"
VWW_PERSON_DATA_DIR = VWW_DATA_DIR / "person"
IC_DATA_DIR = DATA_DIR / "cifar-10-patches-py"
AD_DATA_DIR = DATA_DIR / "ToyCar"

def read_kws_file(path):
    with open(path, mode="rb") as file:
        content = file.read()
    return content

def run_kws():
    # Find all files in the KWS data directory
    #items = os.listdir(KWS_DATA_DIR)
    #files = [item for item in items if os.path.isfile(os.path.join(KWS_DATA_DIR, item))]
    df = pd.read_csv(KWS_DATA_DIR / "y_labels.csv", names=["filename", "no_classes", "class"])
    print(df.to_string())
    for (i, filename) in enumerate(df["filename"]):
        label = df["class"][i]
        data = read_kws_file(KWS_DATA_DIR / filename)
        for x in data:
            print("{:x}".format(x))

def read_vww_file(path):
    image = Image.open(path)
    image.show()
    byte_array = io.BytesIO()
    image.save(byte_array, format="PNG")
    return byte_array.getvalue()




def run_vww():
    items = os.listdir(VWW_NON_PERSON_DATA_DIR)
    non_persons = [item for item in items if os.path.isfile(os.path.join(VWW_NON_PERSON_DATA_DIR, item))]

    items = os.listdir(VWW_PERSON_DATA_DIR)
    persons = [item for item in items if os.path.isfile(os.path.join(VWW_PERSON_DATA_DIR, item))]

    print(non_persons)
    print(persons)
    print(read_vww_file(VWW_PERSON_DATA_DIR / persons[0]))



def main():
    #run_kws()
    run_vww()

main()
