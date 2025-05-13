from main import count_kmers

SEQ = """AAAATTTTGGGGCCCC"""


def test_one_mer():
    cnt = count_kmers(SEQ, k=1)
    assert cnt == {"A": 4, "T": 4, "G": 4, "C": 4}


def test_two_mer():
    cnt = count_kmers(SEQ, k=2)
    assert cnt == {"AA": 3, "AT": 1, "CC": 3, "GC": 1, "GG": 3, "TG": 1, "TT": 3}


def test_three_mer():
    cnt = count_kmers(SEQ, k=3)
    assert cnt == {
        "AAA": 2,
        "AAT": 1,
        "ATT": 1,
        "TTT": 2,
        "TTG": 1,
        "TGG": 1,
        "GGG": 2,
        "GGC": 1,
        "GCC": 1,
        "CCC": 2,
    }


def test_four_mer():
    cnt = count_kmers(SEQ, k=4)
    assert cnt == {
        "AAAA": 1,
        "AAAT": 1,
        "AATT": 1,
        "ATTT": 1,
        "TTTT": 1,
        "TTTG": 1,
        "TTGG": 1,
        "TGGG": 1,
        "GGGG": 1,
        "GGGC": 1,
        "GGCC": 1,
        "GCCC": 1,
        "CCCC": 1,
    }
