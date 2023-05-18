import sys


def main():
    name = sys.argv[1]
    file_name = f"../../../../src/{name}.rs"
    with open(file_name) as f:
        in_code_block = False
        doc_comment_started = False
        for line in f.readlines():
            if line.startswith("///"):
                doc_comment_started = True
                line = line.strip().replace("/// ", "").replace("///", "")
                if "```" in line:
                    if not in_code_block:
                        line = line.replace("```", "```rust,ignore")
                    in_code_block = not in_code_block

                print(line)
            elif doc_comment_started:
                return


if __name__ == '__main__':
    main()
