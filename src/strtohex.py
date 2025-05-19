import argparse


def main(args: argparse.Namespace):
    parts = args.str.split(" ")
    hex = []
    for part in parts:
        print(len(part))
        if len(part) == 4:
            hex.append("0x" + part[2:])
            hex.append("0x" + part[:2])
        elif len(part) == 2:
            hex.append("0x" + part)

    print("[" + ", ".join(hex) + "]")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("str", type=str, help="The str to convert to hex: i.e.: \"5a4c 5049\"")
    return parser.parse_args()


if __name__ == "__main__":
    main(parse_args())
