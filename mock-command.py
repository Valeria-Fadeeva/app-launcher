"""Mock-программа, демонстрирующая ввод/вывод на консоль"""

from os import linesep

r = 5
for i in range(1,r+1):
    print(f'Вопрос {i} из {r}')
    print('Как вас зовут?')
    name = input()
    print(f'Здравствуйте, {name}!', linesep)
