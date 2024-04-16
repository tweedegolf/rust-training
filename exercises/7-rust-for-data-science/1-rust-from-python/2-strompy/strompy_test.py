import strompy

file = open('op.json', 'rb')

data = file.read()

res = strompy.exec(data)

print(res)

file.close()
