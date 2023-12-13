import io

FILE_PATH="./default users/"
DEBUG_FILENAME="ghosts.mk8tt"

ids150cc = []
ids200cc = []
idsSPDC = []

def open_file(filename):
  global ids150cc,ids200cc,idsSPDC
  with open(filename,encoding="UTF-8") as f:
    try:
      line = f.readline().strip()
      ids150cc = line.split(";")
      for id in ids150cc:
        if id:
          id=int(id)

      line = f.readline().strip()
      ids200cc = line.split(";")
      for id in ids200cc:
        if id:
          id=int(id)

      line = f.readline().strip()
      idsSPDC=line.split(";")
      for id in idsSPDC:
        if id:
          id = int(id)
    except:
      pass


open_file(FILE_PATH+DEBUG_FILENAME)
print(ids150cc)
print(ids200cc)
print(idsSPDC)