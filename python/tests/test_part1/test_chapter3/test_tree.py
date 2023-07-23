import pytest
from part1.chapter3.tree import Tree, Leaf, Branch


def test_exercise325():
    tree = Branch(Branch(Leaf(1), Leaf(2)), Branch(Leaf(3), Leaf(4)))
    assert 4 == tree.maximum()

def test_exercise326():
    tree = Branch(Leaf(1), Branch(Leaf(2), Branch(Leaf(3), Leaf(3))))
    assert 3 == tree.depth()

def test_exercise327():
    tree = Branch(Leaf(1), Branch(Leaf(2), Branch(Leaf(3), Leaf(3))))
    expected = Branch(Leaf("1"), Branch(Leaf("2"), Branch(Leaf("3"), Leaf("3"))))
    assert expected == tree.map(lambda x: str(x))

def test_exercise328():
    size = Branch(Branch(Leaf(1), Leaf(2)), Branch(Leaf(3), Leaf(4)))
    assert 7 == size.size_via_fold()
    depth = Branch(Leaf(1), Branch(Leaf(2), Branch(Leaf(3), Leaf(3))))
    assert 3 == depth.depth_via_fold()
    tree = Branch(Leaf(1), Branch(Leaf(2), Branch(Leaf(3), Leaf(3))))
    expected = Branch(Leaf("1"), Branch(Leaf("2"), Branch(Leaf("3"), Leaf("3"))))
    assert expected == tree.map_via_fold(lambda x: str(x))
