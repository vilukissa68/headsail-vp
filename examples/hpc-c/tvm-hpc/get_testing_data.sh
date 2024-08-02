#!/usr/bin/env sh

TARGET_DIR="dev_data"
mkdir -p $TARGET_DIR

# Anomaly detection
echo "Fetching anomaly detection data"
ANOMALY_DETECTION_URL="https://zenodo.org/records/3727685/files/eval_data_train_fan.zip?download=1"
ZIPFILE="anomaly_detection_data.zip"
curl $ANOMALY_DETECTION_URL -o $ZIPFILE
unzip $ZIPFILE -d $TARGET_DIR
rm $ZIPFILE
curl https://raw.githubusercontent.com/mlcommons/tiny/master/benchmark/evaluation/datasets/ad01/y_labels_alt.csv -o y_labels_alt.csv
mv y_labels_alt.csv $TARGET_DIR

# Keyword spotting
echo "Fetching keyword spotting data"
KEYWORD_SPOTTING_URL="https://codeload.github.com/eembc/energyrunner/tar.gz/main"
curl $KEYWORD_SPOTTING_URL | \
  tar -xvz --strip=2 energyrunner-main/datasets/kws01
mv kws01 $TARGET_DIR

# Visual wake word
echo "Fetching visual wake word data"
VWW_URL="https://www.silabs.com/public/files/github/machine_learning/benchmarks/datasets/vw_coco2014_96.tar.gz"
VWW_TAR_NAME="vww_data.tar.gz"
curl $VWW_URL -o $VWW_TAR_NAME
tar -xvf $VWW_TAR_NAME -C $TARGET_DIR
rm $VWW_TAR_NAME

# Image classification
echo "Fetching image classfication data"
IMAGE_CLASSIFICATION_URL="https://www.cs.toronto.edu/~kriz/cifar-10-python.tar.gz"
IMAGE_CLASSIFICATION_TAR_NAME="cifar10.tar.gz"
curl $IMAGE_CLASSIFICATION_URL -o $IMAGE_CLASSIFICATION_TAR_NAME
tar -xvf $IMAGE_CLASSIFICATION_TAR_NAME -C $TARGET_DIR
rm $IMAGE_CLASSIFICATION_TAR_NAME
