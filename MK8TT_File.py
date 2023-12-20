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

  def get_times(self,category):
    start_id=None
    end_id=None
    match str(category["cc"]):
      case "150":
        start_id=self.ids150cc[int(category["track"])]
        if int(category["track"])<MK8TT_File.NUM_TRACKS:
          end_id=self.ids150cc[int(category["track"])+1]
        else:
          end_id=self.ids200cc[0]
      case "200":
        start_id=self.ids200cc[int(category["track"])]
        if int(category["track"])<MK8TT_File.NUM_TRACKS:
          end_id=self.ids200cc[int(category["track"])+1]
        else:
          end_id=self.idsSPDC[0]
      case "spdc":
        start_id=self.idsSPDC[int(category["track"])]
        if int(category["track"])<MK8TT_File.NUM_SPDC_CATEGORIES:
          end_id=self.idsSPDC[int(category["track"])+1]
        else:
          end_id=len(self.filelines)# TODO add custom categories
      #TODO add custom categories
    out = []
    for i in range(start_id,end_id):
      line = self.filelines[i].split(";")
      entry = {}
      entry["time"] = line[0]
      entry["date"] = line[1]
      entry["char"] = line[2]
      entry["kart"] = line[3]
      entry["wheel"]= line[4]
      entry["wing"] = line[5]
      entry["extra"]= line[6]
      out.append(entry)
    return out
  
  def _add_entry(self,category,entry):
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
      line_composition = str(entry["time"])+";"+str(entry["date"])+";"+\
        str(entry["char"])+";"+str(entry["kart"])+";"+\
        str(entry["wheel"])+";"+str(entry["wing"])+";"+\
        str(entry["extra"])+"\n"
      self.filelines.insert(line_num,line_composition)
    else:
      print("error, file not modified")

  def add_new_time(self,category,entry):
    entry["date"]=date.today()
    self._add_entry(self,category,entry)

  def get_time(self,category,entry_id=0):
    try:
      index = None
      match str(category["cc"]):
        case "150":
          index = self.ids150cc[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_TRACKS:
            if index>self.ids150cc[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>self.ids200cc[0]:
              raise "id invalid"
        case "200":
          index = self.ids200cc[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_TRACKS:
            if index>self.ids200cc[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>self.idsSPDC[0]:
              raise "id invalid"
        case "spdc":
          index = self.idsSPDC[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_SPDC_CATEGORIES:
            if index>self.idsSPDC[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>len(self.filelines): # TODO custom categories
              raise "id invalid"
        # TODO custom categories
      if index:
        line = self.filelines[index].split(";")
        entry = {}
        entry["time"] = line[0]
        entry["date"] = line[1]
        entry["char"] = line[2]
        entry["kart"] = line[3]
        entry["wheel"]= line[4]
        entry["wing"] = line[5]
        entry["extra"]= line[6]
        return entry
    except:
      print("id invalid: cannot get")
    return None

  def _replace_time(self,category,new_entry,entry_id=0):
    try:
      index = None
      match str(category["cc"]):
        case "150":
          index = self.ids150cc[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_TRACKS:
            if index>self.ids150cc[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>self.ids200cc[0]:
              raise "id invalid"
        case "200":
          index = self.ids200cc[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_TRACKS:
            if index>self.ids200cc[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>self.idsSPDC[0]:
              raise "id invalid"
        case "spdc":
          index = self.idsSPDC[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_SPDC_CATEGORIES:
            if index>self.idsSPDC[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>len(self.filelines): # TODO custom categories
              raise "id invalid"
        # TODO custom categories
      if index:
        line_composition = str(new_entry["time"])+";"+str(new_entry["date"])+";"+\
        str(new_entry["char"])+";"+str(new_entry["kart"])+";"+\
        str(new_entry["wheel"])+";"+str(new_entry["wing"])+";"+\
        str(new_entry["extra"])+"\n"
        self.filelines[index] = new_entry
    except:
      print("id invalid: cannot get")
      return False

  def edit_time(self,category,replacement_entry,entry_id=0):
    edit_entry = self.get_time(category,entry_id)
    if edit_entry != None:
      if "time" in replacement_entry:
        edit_entry["time"] = replacement_entry["time"]
      if "char" in replacement_entry:
        edit_entry["char"] = replacement_entry["char"]
      if "kart" in replacement_entry:
        edit_entry["kart"] = replacement_entry["kart"]
      if "wheel" in replacement_entry:
        edit_entry["wheel"] = replacement_entry["wheel"]
      if "wing" in replacement_entry:
        edit_entry["wing"] = replacement_entry["wing"]
      if "extra" in replacement_entry:
        edit_entry["extra"] = replacement_entry["extra"]
      self._replace_time(self,category,edit_entry,entry_id)
    else:
      return False

  def delete_time(self,category,entry_id=0):
    try:
      index = None
      match str(category["cc"]):
        case "150":
          index = self.ids150cc[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_TRACKS:
            if index>self.ids150cc[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>self.ids200cc[0]:
              raise "id invalid"
          for n in range(int(category["track"])+1,len(self.ids150cc)):
            self.ids150cc[n]-=1
          for i in range(len(self.ids200cc)):
            self.ids200cc[i]-=1
          for i in range(len(self.idsSPDC)):
            self.idsSPDC[i]-=1
          #TODO custom categories
        case "200":
          index = self.ids200cc[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_TRACKS:
            if index>self.ids200cc[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>self.idsSPDC[0]:
              raise "id invalid"
          for n in range(int(category["track"])+1,len(self.ids200cc)):
            self.ids200cc[n]-=1
          for i in range(len(self.idsSPDC)):
            self.idsSPDC[i]-=1
          #TODO custom categories
        case "spdc":
          index = self.idsSPDC[int(category["track"])]+entry_id
          if int(category["track"])<MK8TT_File.NUM_SPDC_CATEGORIES:
            if index>self.idsSPDC[int(category["track"])+1]:
              raise "id invalid"
          else:
            if index>len(self.filelines): # TODO custom categories
              raise "id invalid"
          for n in range(int(category["track"])+1,len(self.idsSPDCcc)):
            self.idsSPDCcc[n]-=1
          #TODO custom categories
        # TODO custom categories
      del self.filelines[index]
      return True
    except:
      print("id invalid: not deleting")
      return False

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
