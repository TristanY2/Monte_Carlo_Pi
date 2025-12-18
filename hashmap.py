sas = "loveleetcode"


def primera_sin_repetir(s: str) -> int:
    vistos = {}
    for letra in s:
        vistos[letra] = vistos.get(letra, 0) + 1

    for i, letra in enumerate(vistos):
        # print(letra)
        if vistos[letra] == 1:
            return i
    return -1


print(f"La primera letra sin repetir es: {sas[primera_sin_repetir(sas)]}")
