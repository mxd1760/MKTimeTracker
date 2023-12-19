import io,os
from datetime import date
import useful_consts

FILE_PATH="./default users/"
DEBUG_FILENAME="tests.mk8tt"


class MK8TT_File:
  # filename
  # ids150cc
  # ids200cc
  # idsSPDC
  # filelines
  EXTENTION = ".mk8tt"
  EMPTY_FILE_CONSTANT = 4
  NUM_TRACKS = 96
  NUM_SPDC_CATEGORIES = 36

  # TODO custom categories
  def __init__(self,filename,new=False):
    _,extention = os.path.splitext(filename)
    if extention == MK8TT_File.EXTENTION:
      self.filename = filename
    else:
      self.filename = filename+MK8TT_File.EXTENTION
    if new:
      self.ids150cc=[MK8TT_File.EMPTY_FILE_CONSTANT for x in range(MK8TT_File.NUM_TRACKS)]
      self.ids200cc=[MK8TT_File.EMPTY_FILE_CONSTANT for x in range(MK8TT_File.NUM_TRACKS)]
      self.idsSPDC =[MK8TT_File.EMPTY_FILE_CONSTANT for x in range(MK8TT_File.NUM_SPDC_CATEGORIES)]
      self.filelines = [
        ";".join([str(n) for n in self.ids150cc])+"\n",
        ";".join([str(n) for n in self.ids200cc])+"\n",
        ";".join([str(n) for n in self.idsSPDC])+"\n"
      ]
    else:
      try:
        with open(self.filename,"r",encoding="UTF-8") as f:      
          self.filelines = f.readlines();
          self.ids150cc = [int(x) for x in self.filelines[0].split(";")]
          self.ids200cc = [int(x) for x in self.filelines[1].split(";")]
          self.idsSPDC = [int(x) for x in self.filelines[2].split(";")]
          # TODO custom categories
      except:
        pass
    
      if not self.is_valid():
        print("File is Invalid. Returning new file")
        self.__init__(self.filename,True)

  def add_new_time(self,category,entry):
    line_num=-1
    match str(category["cc"]):
      case "150":
        line_num=self.ids150cc[int(category["track"])]
        for n in range(int(category["track"])+1,len(self.ids150cc)):
          self.ids150cc[n]+=1
        for i in range(len(self.ids200cc)):
          self.ids200cc[i]+=1
        for i in range(len(self.idsSPDC)):
          self.idsSPDC[i]+=1
        #TODO custom categories
      case "200":
        line_num=self.ids200cc[category["track"]]
        for n in range(category["track"]+1,len(self.ids200cc)):
          self.ids200cc[n]+=1
        for i in self.idsSPDC:
          i+=1;
        #TODO custom categories
      case "spdc":
        line_num=self.ids200cc[category["track"]]
        for n in range(category["track"]+1,len(self.idsSPDC)):
          self.idsSPDC[n]+=1
        #TODO custom categories
      #TODO custom categories
    
    if line_num!=-1:
      line_composition = str(entry["time"])+";"+str(date.today())+";"+\
        str(entry["char"])+";"+str(entry["kart"])+";"+\
        str(entry["wheel"])+";"+str(entry["wing"])+";"+\
        str(entry["extra"])+"\n"
      self.filelines.insert(line_num,line_composition)
    else:
      print("error, file not modified")
    
  def save(self):
    if self.is_valid():
      self.filelines[0]=";".join([str(n) for n in self.ids150cc])+"\n"
      self.filelines[1]=";".join([str(n) for n in self.ids200cc])+"\n"
      self.filelines[2]=";".join([str(n) for n in self.idsSPDC])+"\n"
      with open(self.filename,"w",encoding="UTF-8") as f:
        f.write("".join(self.filelines))
    else:
      print("Invalid File: not saved");
  
  def is_valid(self):
    try:
      x=4
      for n in self.ids150cc+self.ids200cc+self.idsSPDC:
        assert n>=x
        x=n
      return True
    except:
      return False
    
  def test(self):
    print(self.ids150cc)
    print(self.ids200cc)
    print(self.idsSPDC)
    self.is_valid()
    self.add_new_time({
      "cc":"150",
      "track":"0",
    },{
      "time":"10:00.000",
      "char":"0",
      "kart":"0",
      "wheel":"0",
      "wing":"0",
      "extra":""    
    })
    #self.save()
      
  

if __name__=="__main__":
  file1=MK8TT_File(FILE_PATH+DEBUG_FILENAME)
  file2=MK8TT_File(FILE_PATH+"Test2")
  file1.test()
  if file2.is_valid():
    file2.save()
