#!/usr/bin/env python3

import sys
import signal
import cv2
import numpy as np
from queue import Queue, Empty
import pyscreenshot as ImageGrab
import datetime

end = False

# --------------------------------------cc
def signal_handler(signal, frame):
    global end
    end = True

# --------------------------------------
cap = cv2.VideoCapture(0)

arquivo = sys.argv[1]
arquivo2 = 'tempo.txt'

# Tenta abrir a webcam, e já falha se não conseguir
if not cap.isOpened():
    print('Não foi possível abrir a web cam.')
    sys.exit(-1)

cap.set(cv2.CAP_PROP_FRAME_WIDTH, 640)
cap.set(cv2.CAP_PROP_FRAME_HEIGHT, 480)
cap.set(cv2.CAP_PROP_FPS, 30)

frame_width = int(cap.get(3))
frame_height = int(cap.get(4))
fps = cap.get(cv2.CAP_PROP_FPS)

# Cria o arquivo de video de saída
fourcc = cv2.VideoWriter_fourcc(*'DIVX')
out = cv2.VideoWriter(arquivo, fourcc, fps, (frame_width,frame_height))

out.set(cv2.CAP_PROP_FPS, 30)
print(fps)



# Captura o sinal de CTRL+C no terminal
signal.signal(signal.SIGINT, signal_handler)
#print('Capturando o vídeo da webcam -- pressione Ctrl+C para encerrar...')

# Processa enquanto o usuário não encerrar (com CTRL+C)
while(not end):
    ret, frame = cap.read()

    

    if ret:
        out.write(frame)
    else:
        print('Oops! A captura falhou.')
        break


# Encerra tudo
print(out.get(cv2.CAP_PROP_FPS))
cap.release()
out.release()
