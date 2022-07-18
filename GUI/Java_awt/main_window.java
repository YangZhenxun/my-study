package GUI.Java_awt;
import java.awt.*;
import java.awt.event.*;

public class main_window {
    public static void main(String[] args){
        Frame frame = new Frame("main_window");
        frame.setVisible(true);
        frame.setSize(300,300);
        windowClose(frame);
    }
    private static void windowClose(Frame frame) {
        frame.addWindowListener(new WindowAdapter(){
            @Override
            public void windowClosing(WindowEvent e) {
                System.exit(0);
            }
        });
    }
}
