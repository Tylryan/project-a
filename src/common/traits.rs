use std::path::PathBuf;

pub trait StringFromPathBuf 
{
    fn to_string(self) -> String;
}

impl StringFromPathBuf for PathBuf 
{
    fn to_string(self) -> String 
    {
        return self.to_str().unwrap().to_string();
    }
}

pub trait StringToPathBuf 
{
    fn to_pathbuf(self) -> PathBuf;
}

impl StringToPathBuf for String
{
    fn to_pathbuf(self) -> PathBuf 
    {
        return PathBuf::from(self);
    }

}
