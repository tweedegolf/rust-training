import pytest

from main import decode_orf, Sequence, all_orfs


def test_decode_one():
    seq = "GCTTAA"
    orf = decode_orf(Sequence(seq), 0)

    assert orf.start == 0
    assert orf.end == len(seq)
    assert orf.decoded == "A*"


def test_orf_decoding():
    seq = (
        "GCTGGGATAGTAGGCACATCCTTAAGATTAATTATCCGAGCCGAACTAGGTC"
        "AACCAGGAACTCTTATTGGTAATGATCAAATCTATAATGTAGTAGTTACAGC"
        "TCACGCTTTTGTAATAATTTTTTTTATGGTCATACCTATTATAATTGGAGGA"
        "TTCGGTAATTGA"
    )

    orf = decode_orf(Sequence(seq), 0)

    assert orf.start == 0
    assert orf.end == len(seq)
    assert orf.decoded == "AGIVGTSLRLIIRAELGQPGTLIGNDQIYNVVVTAHAFVIIFFMVIPIIIGGFGN*"


def test_all_orf_decoding_small():
    seq = "ATGGCTTAAATGAATTAG"

    orfs = all_orfs(Sequence(seq))

    orf = orfs.__next__()
    assert orf.start == 0
    assert orf.end == 9
    assert orf.decoded == "MA*"

    orf = orfs.__next__()
    assert orf.start == 9
    assert orf.end == len(seq)
    assert orf.decoded == "MN*"

    with pytest.raises(StopIteration):
        orfs.__next__()


def test_all_orf_decoding():
    seq = (
        "ATGGCTGGGATAGTAGGCACATCCTTAAGATTAATTATCCGAGCCGAACTAGGTCAACCA"
        "GGAACTCTTATTGGTAATGATCAAATCTATAATGTAGTAGTTACAGCTCACGCTTTTGTA"
        "ATAATTTTTTTTATGGTCATACCTATTATAATTGGAGGATTCGGTAATTGAATGACTGTC"
        "TACCCTCCTTTAGCCGCGGCTATTGCCCACGCAGGAGCTTCTGTTGACATGGGTATTTTT"
        "TCTCTACATCTAGCAGGTGTTTCTTCTATTCTAGGTGCCGTAAATTTTATAACAACAGTA"
        "ATTAATATACGTTCATTTGGTATATCTATAGACCAAATACCCTTATTTGTTTGA"
    )

    orfs = all_orfs(Sequence(seq))

    orf = orfs.__next__()
    assert orf.start == 0
    assert orf.end == 171
    assert orf.decoded == "MAGIVGTSLRLIIRAELGQPGTLIGNDQIYNVVVTAHAFVIIFFMVIPIIIGGFGN*"

    orf = orfs.__next__()
    assert orf.start == 171
    assert orf.end == len(seq)
    assert (
        orf.decoded == "MTVYPPLAAAIAHAGASVDMGIFSLHLAGVSSILGAVNFITTVINIRSFGISIDQIPLFV*"
    )
