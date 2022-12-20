import java.util.Scanner;

public class first {
    public static void main(String[] args){
        while (true) {
            int i1 = (int)(Math.random()*10);
            int i2 = (int)(Math.random()*10);
            System.out.print(i1 + "+" + i2 + "=");
            try{
                Scanner scanf = new Scanner(System.in);
                int input = scanf.nextInt();
                int math = i1 + i2;
                if (input == math){
                    System.out.println("你答对了！");
                } else {
                    System.out.println("你答错了！");
                    System.out.println("正确答案是：" + math);
                }
            }
            catch(Exception e){
                System.out.println("请输入数字！");
            }
        }
    }
}
