package com.ui.test;

import javafx.application.Application;

import javafx.stage.Stage;

import javafx.scene.*;

public class windowtest extends Application{
    @Override
    public void start(Stage window) throws Exception {
        Group group = new Group();
        Scene scene = new Scene(group);
        window.setScene(scene);
        window.setTitle("Test");
        window.show();
    }
}
