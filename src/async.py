def gen():
    x = yield "dame algo"
    print("Recibí:", x)
    y = yield "otra cosa"
    print("Recibí:", y)

g = gen()
print(next(g))      # inicia, imprime "dame algo"
print(g.send(0))
print(next(g))   # envía 10 al yield anterior, imprime "Recibí: 10"
