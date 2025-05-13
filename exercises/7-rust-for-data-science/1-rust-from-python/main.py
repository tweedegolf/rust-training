import pprint
from collections import defaultdict
from dataclasses import dataclass
from itertools import batched
from typing import Dict, Iterator

from data import CRAB, DNA_TO_AMINO


# 0. Setup
#
# $ cd rust-dna
# $ uv run maturin develop --uv
# $ cd ..
# $ uv run pytest

# 1. replace this function with Rust
def count_kmers(sequence: str, k: int = 3) -> Dict[str, int]:
    """
    Count the sequences of length `k` in the sequence

    This will return a dictionary containing the counts of a given subsequence in the sequence.

    For example, for the sequence "ATGC":
    * and `k=1` there will be four "one-mers": One of each "A", "T", "G", and "C".
    * and `k=2` there will be three "two-mers": One of each "AT", "TG", and "GC".
    * and `k=3` there will be two "three-mers": One of each "ATG", and "TGC".
    * and `k=4` there will be one "four-mer": "ATGC".

    :param sequence: The sequence to count occurrences in
    :param k: The length of k-mers to count
    :return: A dictionary containing the counts of each subsequence in the sequence
    """
    kmers = defaultdict(int)
    for i in range(len(sequence) - k + 1):
        kmer = sequence[i: i + k]
        assert len(kmer) == k
        kmers[kmer] += 1

    return dict(kmers)


# 2. Translate this check to Rust
def assert_valid_dna(sequence: str):
    if not all(x in "ATGC" for x in sequence):
        raise ValueError("Sequence contains invalid characters")


# 3. Translate this class to Rust
class Sequence:
    sequence: str

    def __init__(self, sequence: str) -> None:
        """
        Initialize a new Sequence object.

        :param sequence: The DNA sequence string.
        :exception ValueError: If the sequence is not a DNA string i.e., contains letters other than A, T, G, C
        """

        assert_valid_dna(sequence)

        self.sequence = sequence

    def __len__(self) -> int:
        return len(self.sequence)

    def __str__(self) -> str:
        return self.sequence

    def kmers(self, k: int = 3) -> Dict[str, int]:
        return count_kmers(self.sequence, k)


# 4. Implement a Rust struct that converts this dataclass from/to Python (https://pyo3.rs/v0.24.2/conversions/traits.html)
#    Extra: Implement __eq__, __hash__, etc. with minimal code (see https://pyo3.rs/v0.24.2/class.html#customizing-the-class)
@dataclass
class OpenReadingFrame:
    start: int
    end: int
    decoded: str


# 5. Translate these functions to Rust
def decode_orf(sequence: Sequence, start: int) -> OpenReadingFrame:
    sub_seq = str(sequence)[start:]
    decoded = str()

    for a, b, c in batched(sub_seq, 3):
        amino = DNA_TO_AMINO[a + b + c]
        if amino == "STOP":
            decoded += "*"
            break
        decoded += amino

    return OpenReadingFrame(decoded=decoded, start=start, end=start + 3 * len(decoded))


def all_orfs(sequence: Sequence) -> Iterator[OpenReadingFrame]:
    i = 0
    while i < len(sequence):
        if str(sequence)[i:].startswith("ATG"):
            orf = decode_orf(sequence, i)
            i = orf.end
            yield orf
            continue
        i += 1


def main():
    cnt = count_kmers(CRAB)

    print("This crab sequence contains the following three-mers:")
    pprint.pp(cnt, sort_dicts=True)

    print("This crab has the following longer proteins encoded:")
    seq = Sequence(CRAB)
    for orf in all_orfs(seq):
        if len(orf.decoded) > 10:
            print(orf.decoded)


if __name__ == "__main__":
    main()
