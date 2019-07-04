#!/usr/bin/env python3
#

import sys
import argparse
import cv2
import numpy as np
import math
from collections import OrderedDict
from collections import deque

def soma_tempo(tempo, soma):
    [ano, mes, dia, hora, minuto, segundo, nano] = tempo
    soma *= 1000000
    nano += soma
    segundo += int(nano/1000000000)
    nano -= int(nano/1000000000)*1000000000
    minuto += int(segundo/60)
    segundo -= int(segundo/60)*60
    hora += int(minuto/60)
    minuto -= int(minuto/60)*60
    dia += int(hora/24)
    hora -= int(hora/24)*24
    mes += int(dia/30)
    dia -= int(dia/30)*30
    ano += int(mes/12)
    mes -= int(mes/12)*12

    return (ano, mes, dia, hora, minuto, segundo, nano)

def tempo_esta_entre(tempo, soma, referencia):
    (ano, mes, dia, hora, minuto, segundo, nano) = soma_tempo(tempo, soma)
    (ano2, mes2, dia2, hora2, minuto2, segundo2, nano2) = referencia
    (ano3, mes3, dia3, hora3, minuto3, segundo3, nano3) = tempo
    texto = str(tempo) + '<' + str(referencia) + '<' + str(soma_tempo(tempo,soma))
    #print(texto)

    maior = False
    if (ano2 > ano):
        maior = True
    elif (ano2 == ano):
        if (mes2 > mes):
            maior = True
        elif (mes2 == mes):
            if (dia2 > dia):
                maior = True
            elif (dia2 == dia):
                if (hora2 > hora):
                    maior = True
                elif (hora2 == hora):
                    if (minuto2 > minuto):
                        maior = True
                    elif (minuto2 == minuto):
                        if (segundo2 > segundo):
                            maior = True
                        elif (segundo2 == segundo):
                            if (nano2 > nano):
                                maior = True

    menor = False
    if (ano3 > ano2):
        menor = True
    elif (ano3 == ano2):
        if (mes3 > mes2):
            menor = True
        elif (mes3 == mes2):
            if (dia3 > dia2):
                menor = True
            elif (dia3 == dia2):
                if (hora3 > hora2):
                    menor = True
                elif (hora3 == hora2):
                    if (minuto3 > minuto2):
                        menor = True
                    elif (minuto3 == minuto2):
                        if (segundo3 > segundo2):
                            menor = True
                        elif (segundo3 == segundo2):
                            if (nano3 > nano2):
                                menor = True

    return (not menor) and (not maior)


def normalizar(valor):
    maximo = 0

    for x in range(len(valor)):
        if maximo < valor[x]:
            maximo = valor[x]
    
    for x in range(len(valor)):
        if maximo > 0:
            valor[x] = valor[x]/maximo
    
    return valor

def get_data(linha):
    [ano, mes, dia, hora, minuto, segundo, nano] = linha.split('/')
        
    nano = int(nano)
    segundo = int(segundo)
    minuto = int(minuto)
    hora = int(hora)
    dia = int(dia)
    mes = int(mes)
    ano = int(ano)

    return (ano, mes, dia, hora, minuto, segundo, nano)


def ler_saidas(linha):
    [data, label, valor_txt] = linha.split('-')
    [ano, mes, dia, hora, minuto, segundo, nano] = data.split('/')
        
    nano = int(nano)
    segundo = int(segundo)
    minuto = int(minuto)
    hora = int(hora)
    dia = int(dia)
    mes = int(mes)
    ano = int(ano)
    valor_txt = valor_txt.strip("\n")
    if valor_txt == "true":
        valor = 1.0
    elif valor_txt == "false":
        valor = 0.0
    else:
        valor = float(valor_txt)
    return ((ano, mes, dia, hora, minuto, segundo, nano), label, valor)
            
#---------------------------------------------
def main(argv):

    """
    Main entry of this script.

    Parameters
    ------
    argv: list of str
        Arguments received from the command line.
    """

    nome_arquivo = './Testes/{}.kans'.format(sys.argv[1])
    numero_sessao = sys.argv[2]
    data_inicio = get_data(sys.argv[3])
    milisegunds = int(sys.argv[4])
    num_saidas = int(sys.argv[5])

    pontos = int((1910 - 200)/5)
    resolucao = math.ceil(milisegunds/pontos)

    dados = []
    dicionario = {}

    labels = []
    saida = []
    checagem = []
    valores = []


    for x in range(num_saidas):
        label = sys.argv[6+x]
        labels.append(sys.argv[6+x])
        lista = []
        lista2 = [0.0]
        dados.append(deque(lista))
        dicionario.update({sys.argv[6+x]:x})
        checagem.append(False)
        valores.append(deque(lista2))

    arquivo = open(nome_arquivo,"r")
    linha = arquivo.readline()

    while True:
        while ".Sessao:" not in linha:
            linha = arquivo.readline()
        linha = arquivo.readline()
        if linha.strip("\n") == numero_sessao:
            break


    while "Processado" not in linha:
        linha = arquivo.readline()
    
    linha = arquivo.readline().strip("\n")

    while "." != linha and ".Sessao:" != linha:
        (tempo, tipo, valor) = ler_saidas(linha)

        if not (tipo in labels):
            print("Pulou")
            linha = arquivo.readline().strip("\n")
            continue

        dados[dicionario[tipo]].append((tempo, valor))

        linha = arquivo.readline().strip("\n")
    arquivo.close()

    for x in range(num_saidas):
        for i in range(pontos):
            if len(dados[x]) > 0:
                (tempo, valor) = dados[x][0]
                saida = 0.0
                divisor = 0
                while tempo_esta_entre(soma_tempo(data_inicio,i*resolucao), resolucao, tempo):
                    saida += valor
                    dados[x].rotate(-1)
                    dados[x].pop()
                    divisor += 1
                    if len(dados[x]) == 0:
                        break
                    (tempo, valor) = dados[x][0]
                if divisor == 0:
                    saida = valores[x][-1]
                else:
                    saida /= divisor
            else:
                saida = valores[x][-1]

            valores[x].append(saida)


    for i in range(num_saidas):
        valores[i] = normalizar(valores[i])

    print(dicionario)
    
     #Create a black image
    img = np.zeros((1080,1920,3), np.uint8)
    font = cv2.FONT_HERSHEY_SIMPLEX
    branco = (255,255,255)
    amarelo = (0,255,255)
    tamanho = 19

    tam_linha = int(950/num_saidas)
    multiplicador = int(8/num_saidas)


    #Desenhar frame
    cv2.putText(img, '{} sessao {}'.format(sys.argv[1], numero_sessao), (10,30), font, 1, branco, 2)
    for i in range(num_saidas):
        cv2.putText(img,labels[i], (10, 40 + int(tam_linha/2) + i*tam_linha), font, 1, branco, 3)
        #cv2.putText(img,'0',(185,160 + tam_linha + i*tam_linha), font, 0.5, branco)
        cv2.putText(img,'0',(185,40 + tam_linha + i*tam_linha), font, 0.5, branco)
        #cv2.putText(img,'0.5',(170,108 + int(tam_linha/2) + i*tam_linha), font, 0.5, branco)
        cv2.putText(img,'0.5',(170,40 + int(tam_linha/2) + i*tam_linha), font, 0.5, branco)
        cv2.putText(img,'1',(185,65 + i*tam_linha), font, 0.5, branco)
        #Linha vertical:
        cv2.line(img,(200,40 + i*tam_linha),(200,40 + tam_linha + i*tam_linha), branco, 2)
        #Linha Horizontal:
        cv2.line(img, (200, 40 + tam_linha + i*tam_linha), (1910, 40 + tam_linha + i*tam_linha), branco, 3)
    cv2.putText(img,'minuto:segundo:decimo de segundo',(950,1070), font, 0.5, branco)

    contagem = 1
    divisoes = (int)(pontos/tamanho)

    for i in range(pontos - 1):
        for x in range(num_saidas):
            valor1 = valores[x][0]
            valor2 = valores[x][1]
            valores[x].rotate(-1)
            valores[x].pop()

            ponto1 = (200 + int(i*5), 40 + tam_linha + x*tam_linha - int(valor1*(tam_linha - 5)))
            ponto2 = (200 + int((i+1)*5), 40 + tam_linha + x*tam_linha - int(valor2*(tam_linha - 5)))
            cv2.line(img, ponto1, ponto2, amarelo, 2)
        if i  == contagem*divisoes:
            contagem +=1
            (ano, mes, dia, hora, minuto, segundo, nano) = soma_tempo(data_inicio, i*resolucao)
            cv2.putText(img,'{}:{}:{}'.format(minuto, segundo, int(nano/10000000)), (150 + int(i*5), 1015), font, 0.5, branco, 1)
            for j in range(num_saidas):
                cv2.line(img,(200 + int((i+1)*5), 40 + j*tam_linha), (200 + int((i+1)*5), 40 + tam_linha + j*tam_linha), branco, 1)
            
    nome_saida = './Saida/Graficos/{}_{}.png'.format(sys.argv[1], numero_sessao)

    cv2.imwrite(nome_saida, img)
    cv2.destroyAllWindows()

#---------------------------------------------
# namespace verification for invoking main
#---------------------------------------------
if __name__ == '__main__':
    main(sys.argv)
 
