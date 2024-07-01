#!/usr/bin/env python3

import argparse
import numpy as np
import torch

def hex_to_unsigned_int(hex_str, bit_length=32):
    """Convert a hexadecimal string to a signed integer."""
    num = int(hex_str, 16)
    return num

def hex_to_signed_int(hex_str, bit_length=32):
    """Convert a hexadecimal string to a signed integer."""
    num = int(hex_str, 16)
    max_unsigned = 2 ** bit_length
    max_signed = 2 ** (bit_length - 1)
    if num >= max_signed:
        num -= max_unsigned
    return num

def convert_file_by_column(input_file, signed, bit_length=32):
    """Convert hex values in a file to signed or unsigned integers column by column and save to a new file."""
    output_file = input_file.replace(".mem", "_by_column.rs")
    with open(input_file, 'r') as infile:
        lines = infile.readlines()

    result = []
    hex_values = [line.split() for line in lines]
    columns = list(zip(*hex_values))

    for column in columns:
        if signed:
            ints = [hex_to_signed_int(hex_value, bit_length) for hex_value in column]
        else:
            ints = [hex_to_unsigned_int(hex_value, bit_length) for hex_value in column]
        print(ints)
        result += ints

    return result

def convert_file(input_file, signed, bit_length=32):
    """Convert hex values in a file to signed integers and save to a new file."""
    output_file = input_file.replace(".mem", ".rs")
    result = []

    with open(input_file, 'r') as infile:
        lines = infile.readlines()

    for line in lines:
        hex_values = line.split()
        if signed:
            ints = [hex_to_signed_int(hex_value, bit_length) for hex_value in hex_values]
        else:
            ints = [hex_to_unsigned_int(hex_value, bit_length) for hex_value in hex_values]
            print(ints)
            result += ints

    return result


def main():
    parser = argparse.ArgumentParser(description='Convert hexadecimal values in a file to signed decimal integers.')
    parser.add_argument('din', help='The input file containing hexadecimal values.')
    parser.add_argument('wgt', help='The kernel file containing hexadecimal values.')
    #parser.add_argument('dout', type=str, default='', help='The input file containing hexadecimal values.')
    parser.add_argument('--signed', type=bool, default=False, action=argparse.BooleanOptionalAction, help='Interpreted values as signed (default: false).')
    parser.add_argument('--bit_length', type=int, default=32, help='The bit length of the signed integers (default: 32).')
    parser.add_argument('--by_column', action=argparse.BooleanOptionalAction, default=False, help='Process the file column by column (default: false).')

    args = parser.parse_args()

    # Load input data
    with open(args.din, 'r') as din_file:
        din_hex = din_file.read().replace("\n", "").split(" ")

    din_hex = din_hex[:-1]
    din = np.array([int(value, 16) for value in din_hex], dtype=np.int8)

    # Load wgt data
    with open(args.wgt, 'r') as wgt_file:
        wgt_hex = wgt_file.read().replace("\n", "").split(" ")

    wgt_hex = list(filter((lambda x : x !=  ''), wgt_hex))
    wgt = np.array([int(value, 16) for value in wgt_hex], dtype=np.int8)

    din = torch.tensor(din)
    wgt = torch.tensor(wgt)

    din = din.reshape(1,3,10,15)
    wgt = wgt.reshape(16,3,3,3)

    print(din)
    print(wgt)

    x = torch.randn(1,1,4,4);
    y = torch.randn(1,1,4,4);
    z = torch.nn.functional.conv2d(x,y);
    print(z)

    # for kernel in wgt:
    #     filt = np.zeros((8,13))
    #     for (channel_idx, channel_k) in enumerate(kernel):
    #         print("Channel k:", channel_k.shape, "kernel:", kernel.shape, "channel_din:", din[channel_idx].shape)
    #         res = signal.convolve2d(din[channel_idx], channel_k, mode='valid')
    #         filt += res
    #     print(filt.size)
    #     no_output += filt.size
    # print(no_output)
    out = torch.nn.functional.conv2d(din, wgt, padding="valid")
    print(out)


if __name__ == '__main__':
    main()
