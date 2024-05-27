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

def convert_file_by_column(input_file, signed, bit_length=32):
    """Convert hex values in a file to signed or unsigned integers column by column and save to a new file."""
    output_file = input_file.replace(".mem", "_by_column.rs")
    with open(input_file, 'r') as infile:
        lines = infile.readlines()

    hex_values = [line.split() for line in lines]
    columns = list(zip(*hex_values))

    with open(output_file, 'w') as outfile:
        sign = "i" if signed else "u"
        outfile.write(f'pub const DATA: &[{sign}{bit_length}] = &[\n')
        for column in columns:
            if signed:
                ints = [hex_to_signed_int(hex_value, bit_length) for hex_value in column]
            else:
                ints = [hex_to_unsigned_int(hex_value, bit_length) for hex_value in column]
            outfile.write(', '.join(map(str, ints)))
            outfile.write(',\n')
        outfile.write('];')

def convert_weight_file(input_file, signed, bit_length=8):
    output_file = input_file.replace(".mem", ".rs")
    with open(input_file, 'r') as infile:
        lines = infile.readlines()

    lines = [line.replace("\n", "").split("  ") for line in lines]
    array = []
    for j in range(16):
        for (i, _) in enumerate(lines):
            print(lines[i][j].split(" "))
            array.append(lines[i][j].split(" "))
    print(array)

    with open(output_file, 'w') as outfile:
        sign = "i" if signed else "u"
        outfile.write(f'pub const DATA: &[{sign}{bit_length}] = &[\n')
        for sub_row in array:
            if signed:
                ints = [hex_to_signed_int(hex_value, bit_length) for hex_value in sub_row]
            else:
                ints = [hex_to_unsigned_int(hex_value, bit_length) for hex_value in sub_row]
            outfile.write(', '.join(map(str, ints)))
            outfile.write(',\n')
        outfile.write('];')



def main():
    parser = argparse.ArgumentParser(description='Convert hexadecimal values in a file to signed decimal integers.')
    parser.add_argument('input_file', help='The input file containing hexadecimal values.')
    parser.add_argument('--signed', type=bool, default=False, action=argparse.BooleanOptionalAction, help='Interpreted values as signed (default: false).')
    parser.add_argument('--bit_length', type=int, default=32, help='The bit length of the signed integers (default: 32).')
    parser.add_argument('--by_column', action=argparse.BooleanOptionalAction, default=False, help='Process the file column by column (default: false).')
    parser.add_argument('--weight', action=argparse.BooleanOptionalAction, default=False, help='Process a weight file (default: false).')

    args = parser.parse_args()
    print(args)

    if args.weight:
        convert_weight_file(args.input_file, args.signed)
    elif args.by_column:
        convert_file_by_column(args.input_file, args.signed, args.bit_length)
    else:
        convert_file(args.input_file, args.signed, args.bit_length)
    print(f"Conversion complete. Signed decimal values saved to {args.input_file.replace(".mem", ".rs")}.")

if __name__ == '__main__':
    main()

