fn main() {
   
   enum IpAddrType {
    //define that an Ip could be either V4 or V6
       V4,
       V6,
   }

   enum IpAddr{
    //define that an Ip could be either V4 or V6
       V4(u8, u8, u8, u8),
       V6(String),
   }



   let home = IpAddr::V4(127,0,0,1);
   let loopback = IpAddr::V6(String::from("::1"));





   let ip_v4 = IpAddrType::V4; // <- :: is used to access the enum variant, it means that the variant is inside the IpAddrType enum type namespace
    let ip_v6 = IpAddrType::V6;

    // we can use those enums as a parameters in a function

    fn router(ip_type: IpAddrType){

    }

        router(IpAddrType::V4);
        router(IpAddrType::V6);

    
}

fn with_structs(){
   
    enum IpAddrType {
           V4,
           V6,
       }    

    // we can also use enums with structs

    struct IpAddr {
        ip_type: IpAddrType,
        address: String,
    }
 
    let _home = IpAddr {
        ip_type: IpAddrType::V4,
        address: String::from("127.0.0.1"),
     };
 
     let _loopback = IpAddr {
         ip_type: IpAddrType::V6,
         address: String::from("::1"),
      };

}