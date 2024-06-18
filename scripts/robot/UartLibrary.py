#!/usr/bin/env python3
import serial

class UartLibrary:
    def __init__(self, port, baudrate):
        self.port = port
        self.baudrate = baudrate

    # Reads binary file and writes contents as bytes to uart
    def read_file_and_write_bin_to_uart(self, path):
        ser = serial.Serial(self.port, self.baudrate)
        with open(path, 'rb') as file:
            data = file.read()
            ser.write(data)
        ser.close()

    # Reads mem file (hex formatted bytes as strings) and writes the values to uart
    def read_file_and_write_mem_to_uart(self, path):
        ser = serial.Serial(self.port, self.baudrate)
        with open(path, 'r') as file:
            content = file.read()

        cleaned_content = ''.join(content.split())
        byte_stream = bytes.fromhex(cleaned_content)
        ser.write(byte_stream)
        ser.close()
