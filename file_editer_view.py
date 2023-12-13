from tkinter import *
from tkinter import ttk

# def calculate(*args):
#   try:
#     value = float(feet.get())
#     meters.set(int(0.3048 *value* 10000.0 + 0.5)/10000.0)
#   except ValueError:
#     pass


def add_entry(*args):
  try:
    print(f"{minutes.get()}:{seconds.get()}.{thousandths.get()};  {year.get()}-{month.get()}-{day.get()};  {extra_data.get()}")
  except ValueError:
    pass

root = Tk()
root.title("File Editor")
#iconbitmap


mainframe = ttk.Frame(root, padding="3 3 12 12")
mainframe.grid(column=0, row=0, sticky = (N, W, E, S))
root.columnconfigure(0,weight=1)
root.rowconfigure(0,weight=1)


minutes=StringVar()
minutes_entry=ttk.Entry(mainframe,width=1,textvariable=minutes)
minutes_entry.grid(column=1,row=1,sticky=(W,E))

seconds=StringVar()
seconds_entry=ttk.Entry(mainframe,width=2,textvariable=seconds)
seconds_entry.grid(column=2,row=1,sticky=(W,E))

thousandths=StringVar()
seconds_entry=ttk.Entry(mainframe,width=3,textvariable=thousandths)
seconds_entry.grid(column=3,row=1,sticky=(W,E))

year=StringVar()
year_entry=ttk.Entry(mainframe,width=4,textvariable=year)
year_entry.grid(column=4,row=1,sticky=(W,E))

month=StringVar()
month_entry=ttk.Entry(mainframe,width=2,textvariable=month)
month_entry.grid(column=5,row=1,sticky=(W,E))

day=StringVar()
day_entry=ttk.Entry(mainframe,width=2,textvariable=day)
day_entry.grid(column=6,row=1,sticky=(W,E))

extra_data=StringVar()
extra_data_entry=ttk.Entry(mainframe,width=20,textvariable=extra_data)
extra_data_entry.grid(column=7,row=1,sticky=(W,E))

ttk.Button(mainframe,text="Add Entry",command=add_entry).grid(column=8,row=1,sticky=(W,E))

root.mainloop()
