package com.additation.first;
import java.util.Scanner;

/**
 * Hello world!
 *
 */
public class App
{
    public static void main( String[] args ){
        while (true){
            int i1 = (int)(Math.random()*10+1);
            int i2 = (int)(Math.random()*10+1);
            System.out.print(i1 + "+" + i2 + "=");
            try{
                Scanner scan = new Scanner(System.in);
                int input = scan.nextInt();
                int math = i1 + i2;
                if (input == math){
                    System.out.println("你答对了！");
                } else {
                    System.out.println("你答错了！");
                    System.out.println("正确答案是："+ math);
                }
            } catch (Exception _e){
                System.out.println("请输入数字！");
            }
        }
    }
}
