import liborderbooklib

print(liborderbooklib.create_buy_order(4))
print(liborderbooklib.create_buy_order(8))

print(liborderbooklib.step())
print(liborderbooklib.step())
print(liborderbooklib.step())
print(liborderbooklib.step())

print(liborderbooklib.get_order_count())
print(liborderbooklib.get_buy_orderbook())
