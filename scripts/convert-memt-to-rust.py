import argparse

def hex_to_signed_int(hex_str, bit_length=32):
    """Convert a hexadecimal string to a signed integer."""
    num = int(hex_str, 16)
    max_unsigned = 2 ** bit_length
    max_signed = 2 ** (bit_length - 1)
    if num >= max_signed:
        num -= max_unsigned
    return num

def hex_to_unsigned_int(hex_str, bit_length=32):
    """Convert a hexadecimal string to a signed integer."""
    num = int(hex_str, 16)
    return num

def convert_file(input_file, signed, bit_length=32):
    """Convert hex values in a file to signed integers and save to a new file."""
    output_file = input_file.replace(".mem", ".rs")
    with open(input_file, 'r') as infile, open(output_file, 'w') as outfile:
        sign = "i" if signed else "u"
        outfile.write('pub const DATA: &[{sign}{bit_length}] = &[\n'.format(sign=sign, bit_length=bit_length))
        for line in infile:
            hex_values = line.split()
            if signed:
                ints = [hex_to_signed_int(hex_value, bit_length) for hex_value in hex_values]
            else:
                ints = [hex_to_unsigned_int(hex_value, bit_length) for hex_value in hex_values]
            outfile.write(', '.join(map(str, ints)))
            outfile.write(',\n')
        outfile.write('];')


def main():
    parser = argparse.ArgumentParser(description='Convert hexadecimal values in a file to signed decimal integers.')
    parser.add_argument('input_file', help='The input file containing hexadecimal values.')
    parser.add_argument('--signed', type=bool, default=False, action=argparse.BooleanOptionalAction, help='Interpreted values as signed (default: false).')
    parser.add_argument('--bit_length', type=int, default=32, help='The bit length of the signed integers (default: 32).')

    args = parser.parse_args()
    print(args)

    convert_file(args.input_file, args.signed, args.bit_length)
    print(f"Conversion complete. Signed decimal values saved to {args.input_file.replace(".mem", ".rs")}.")

if __name__ == '__main__':
    main()

