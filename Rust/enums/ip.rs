fn main()
{
    enum IpAddrKind
    {
        V4, 
        V6, 
        V8,

    }

    struct IpAddr 
    {
        kind:IpAddrKind,
        address:String,
    }

    let home = IpAddr
    {
        kind:IpAddrKind::V4, 
        address:String::from("127.0.0.1"),
    };

    let loopback = IpAddr
    {
        kind:IpAddrKind::V6, 
        address:String::from("::1"),
    };

    let home2 = IpAddr
    {
        kind:IpAddrKind::V8,
        address:String::from("::2"),
    };
}

