from technicalysis.core import add

def aggregate(values):
    """
    Agrège une liste de valeurs en les additionnant avec la fonction add.
    
    Exemple:
    >>> aggregate([1, 2, 3, 4])
    10
    """

    result = 0
    for value in values:
        result = add(result, value)
    return result

__all__ = ["aggregate", "add"]