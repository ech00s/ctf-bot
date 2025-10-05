use crate::models::Ctf;

pub fn get_ctfs()->Vec<Ctf>{
    vec![
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"linux-i".to_string(),
            objective:"hidden file".to_string(),
            flag:"FLAG1_31337".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"linux-ii".to_string(),
            objective:"hidden file".to_string(),
            flag:"FLAG2_42448".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"linux-iii".to_string(),
            objective:"grep tmp log".to_string(),
            flag:"FLAG3_55352".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"base64".to_string(),
            objective:"base64 decode".to_string(),
            flag:"FLAG4_63992".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"linux-iv".to_string(),
            objective:"use nc".to_string(),
            flag:"FLAG5_11314".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"user-id".to_string(),
            objective:"from user".to_string(),
            flag:"FLAG6_41442".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"linux-vi".to_string(),
            objective:"from user dir flags".to_string(),
            flag:"FLAG7_55241".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"networking".to_string(),
            objective:"routing table".to_string(),
            flag:"172.17.0.1".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"web-service".to_string(),
            objective:"from web service".to_string(),
            flag:"FLAG9_99382".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"website".to_string(),
            objective:"from website".to_string(),
            flag:"FLAG10_45776".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"filetype".to_string(),
            objective:"from oddfile".to_string(),
            flag:"FLAG12_552412".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"nmap".to_string(),
            objective:"from nmap".to_string(),
            flag:"Elite".to_string()
        },
        Ctf{
            image:"ipvfletch/kiddoctf:latest".to_string(),
            id:"tcpdump".to_string(),
            objective:"from tcpdump".to_string(),
            flag:" FLAG14_13370".to_string()
        },

    ]
}