while (true){
    Random e = new Random();
    int i1 = e.Next(1, 10);
    int i2 = e.Next(1, 10);
    Console.Write(i1 + "+" + i2 + "=");
    try{
        int input = int.Parse(Console.ReadLine());
        int math = i1 + i2;
        if (input == math){
            Console.WriteLine("你答对了！");
        } else {
            Console.WriteLine("你答错了！");
            Console.WriteLine("正确答案是："+math);
        }
    } catch (Exception _e){
        Console.WriteLine("请输入数字！");
    }
}
