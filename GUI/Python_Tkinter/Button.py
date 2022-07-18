import tkinter as tk

a = False
top = tk.Tk(className = " Button")
top.geometry("300x300")
var = tk.StringVar()

lal = tk.Label(top,textvariable= var)
lal.grid(column=0,row = 0)
lal.pack()

def Label():
    global a
    if a == False:
        a = True
        var.set("click Button!")
    else:
        a = False
        var.set("")
Button1 = tk.Button(bd=2,text="Button",command=Label)
Button1.place(x=0,y=0,width=70,height=35)

top.mainloop()
