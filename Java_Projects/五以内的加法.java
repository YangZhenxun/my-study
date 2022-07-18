package Java_Projects;
import java.util.*;

public class 五以内的加法{
    public static void main(String[] args){
        while (true){
            try{
                Scanner input = new Scanner(System.in);
                Random random = new Random();
                int n1 = random.nextInt(5) + 1;
                int n2 = random.nextInt(5) + 1;
                int maths = n1 + n2;
                System.out.print(n1 +"+" + n2 + "=");
                long a = input.nextLong();
                if (a == maths){
                    System.out.println("你答对了!");
                }else{
                    System.out.println("你答错了!");
                    System.out.println("正确答案是" + maths);
                }
            }
            catch (Exception e){
                continue;
            }
        }
    }
}
