import pytest

from main import assert_valid_dna


def test_dna_validation():
    assert_valid_dna("ATGC")

    with pytest.raises(ValueError):
        assert_valid_dna("_")

    with pytest.raises(ValueError):
        assert_valid_dna(" ")
