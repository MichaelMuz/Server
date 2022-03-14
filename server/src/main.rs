/*
our server will be barebones meaning it will work directly with the http and tcp
protocols
tcp - transport layer protocol describing how data is transfered from one computer
to another
http - application level protocol describin ghow the data is structured

if we had a client like a webbrowser requesting a webpage from a server, http
will describe the strcture of the data the client is sending to the server and
the server is sending back to the client so that both know how to interpret the data
and tcp is going to describe how to actually send that data from the client to the
server and vice versa. Devlving into the http and tcp protocols is out of range for
these notes but supplimental material can be found at the description of this
video: https://www.youtube.com/watch?v=BHxmWTVFWxQ&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=42

*/
/*
when we test this we do cargo run then open a browser and navigate to the address
we are listening on, including port #, can use ctr c to stop running
*/

//import filesystem module
use std::fs;
//first thing we need to do is listen to tcp requests so we will bring in
// the tcp listener strcut from the standard library 
use std::net::TcpListener;
//we will use the following to read data from the TcpStream, we will need to
// bring the TcpStream struct into scope
use std::net::TcpStream;

//this contains the buffer.read() method inside so we import all of what is inside
use std::io::prelude::*;


fn main(){
    //lets create a new listener, we use the bind associated function and
    // pass in the ip address we want to bind on and the port number
    // here we use local host and the port 7878 for no real reason
    //bind returns a result type and if this was a production server we 
    // would want to handle that error case gracefully but we are just practicing
    // so we use unwrap
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();
    
    //now that we have a listener at that address listening for connections
    // lets loop through the conncections and print if we have one

    //here we call the .incoming() method on listener which will give us 
    // an iterator over the connections being recieved in the form of a
    // tcp stream, here the stream from the loop is of type Result
    // with either a TcpStream or an Error
    /* 
    for stream in listener.incoming(){
        //we use shadowing to make stream just a TcpStream, we dont handle the
        // error case
        let stream = stream.unwrap();
        println!("connection established");
    }
    //when we run it to test here our string gets printed multiple times
    // and the browser shows an error, this is because our server is not actually
    // returning any data, our string is printed multiple times because web brosers
    // will retry connections when they fail
    */
    //we redo this so we can actually handle the connection
    for stream in listener.incoming(){
        //we use shadowing to make stream just a TcpStream, we dont handle the
        // error case
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}

//will read data from the TcpStream
fn handle_connection_proto(mut stream: TcpStream){

    //create buffer to hold the data that is read
    // this buffer is 1024 bytes long, this is large enought to store the basic
    // requests we are going to working with, but if this was a production
    // server you would want your buffer to be able to handle requests
    // of arbitrary size
    let mut buffer = [0; 1024];

    //we prefix with b which will get us a byte array representing the string
   

    //next we will check if the buffer starts with our string
   

    

    //we call stream.read() and pass in the buffer
    //that is why the stream argument is mutable, the read method takes a mutable
    // reference to self, this is because hwen you read some internal state gets
    // modified
    //this will populate the buffer with data from the stream .read() returns a
    // result type so we .unqrap() for simplicity
    stream.read(&mut buffer).unwrap();

    /*
    //instead of printing, for explination look below, we make a response
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    //next we will write out response to the stream
    //the .write() method accepts a buffer of bytes as an input so we do
    // .as_byte() on responsse, read returns a result method so we call
    // .unwrap(), we have to do this even though nothing captures the output 
    // because you have to handle possible errors
    stream.write(response.as_bytes()).unwrap();
    //now we flush the stream
    //flush will wait until all bytes are written to the connection also returns
    // a result so we again unwrap
    stream.flush().unwrap();
    //with these changes our server will now return a valid reponse
    //we now get a blank page instead of an error page
   
    */
    //instead of a blank response we should work on returning valid html, we 
    // will make a file in the root of our project, not the source folder 
    // called "index.html"

    //here we get the contents of the file and store it in a variable

    let contents = fs::read_to_string("index.html").unwrap();

    //here we make a string using format, this should look similar to the one before
    // but this time since we actually have content we add a Content-Length
    // header, which specifies the amount fo bytes we are returning in the message
    // body, (i guess the length method is adaquate, idk if it retunrs len in byes)

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    //problem here is that we will return this html for any request we will fix this 
    // by adding b"GET / HTTP /1.1\r\n" in the new handle_connection() function
    // below


    //we print the contents of our buffer by calling the from_utf8_lossy()
    // function which converts a slice of bytes into a string including
    // invalid characters, we pass in a slice of our buffer spanning the entire
    // buffer
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    /*
    lets look at the output of printing this, we get multiple of the same output
    as the broswer retries after not getting a connection, lets look at one
    */

//Request: GET / HTTP/1.1
    //http method used, uri and http version number. Get on the root(/) path
    // using http version 1.1, even thought it is not visable the request line
    // ends with a carriage return and a line feed sequence which is basically
    // something that seperates the request line from the rest of the request 
    // data
//Host: 127.0.0.1:7878
//Connection: keep-alive
//Cache-Control: max-age=0
//sec-ch-ua: " Not;A Brand";v="99", "Google Chrome";v="97", "Chromium";v="97"
//sec-ch-ua-mobile: ?0
//sec-ch-ua-platform: "Windows"
//Upgrade-Insecure-Requests: 1
//User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36
//Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
//Sec-Fetch-Site: none
//Sec-Fetch-Mode: navigate
//Sec-Fetch-User: ?1
//Sec-Fetch-Dest: document
//Accept-Encoding: gzip, deflate, br
//Accept-Language: en-US,en;q=0.9
    //underneath the request line we have various request headers and then
    // the request body, since this is a get request there is no body

//now we must write a response which is of the following form:
//HTTP-version Status-Code Reason-Phrase CRLF
//heeaders CRLF
//message-body


//the first line is the status line which consists of the HTTP version, the 
// Status-Code, Reason-Phrase and a carriage return line feed sequence
//o the next line we have headers followed by a crriage return line feed sequence
//then the message body

//here is an example of a request that contains no headers and no message body
//we specify the HTTP version(HTTP/1.1) then the status code (200), 200 is the
// standard status code for success then the reason phrase which will be OK
// then we need 2 carriage return line feed sequences, each composed of (\r\n)
//ex: HTTP/1.1 200 OK\r\n\r\n
    
}

fn handle_connection_proto2(mut stream: TcpStream){

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){

        let contents = fs::read_to_string("index.html").unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();

        stream.flush().unwrap();
    }
    else{
        //if it did not start with the get request begining we will return
        // a 404 error page

        //this line contains the 404 status
        let status_line = "HTTP/1.1 404 NOT FOUND";
        //extract contents from 404.html
        let contents = fs::read_to_string("404.html").unwrap();
        
        //construct response
        let response = format!(
            "{}\r\nContent-Length: {}\n\r\n\r{}",
            status_line,
            contents.len(),
            contents
        );

        //write to stream
        stream.write(response.as_bytes()).unwrap();
        //flush the stream
        stream.flush().unwrap();
    }
    //now if we go to out page we get the normal page but if we put "/" followed
    // by something else, we give a 404 error
    //other issue is that we have a lot of rewritten code since the difference
    // between the if and the else is basically the status line and the html page
}

fn handle_connection(mut stream: TcpStream){

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    //now we want our if else statements to get us a tuple with 2 strings,
    // one with the status line and one with the page
    let (status_line, filename) = 
        //if the buffer starts with the expected request line the
        // the status line will be a 200 and filename will be index.html
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")
        }
        else{
            //if not, status line will be a 404, and the file will be the 404 error
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
      
        
    let contents = fs::read_to_string(filename).unwrap();
    
    
    let response = format!(
        "{}\r\nContent-Length: {}\n\r\n\r{}",
        status_line,
        contents.len(),
        contents
    );

    
    stream.write(response.as_bytes()).unwrap();
  
    stream.flush().unwrap();
}