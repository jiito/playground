package main

import (
       "fmt"
       "log"
       "net/http"
)


func homePage(w http.ResponseWriter, r *http.Request){
       fmt.Fprintf(w, "Welcome to the homepage!")
       fmt.Println("Endpoint hit: Homepage!")
}
