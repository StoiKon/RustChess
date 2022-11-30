use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};
pub struct Selected_Piece{
    pub selected:bool,
    pub moves:Vec<Position>,
    //at moves 0 is going to be the position of the piece
    //in order to select it 
    //without it being inside the struct
}
impl Selected_Piece{
    pub fn new(sel:bool,moves:Vec<Position>) -> Self {
         Self{ selected:sel,
           moves:moves,
         }
    }
    pub fn empty_new() -> Self {
         Self{ selected:false,
           moves:Vec::<Position>::new(),
         }
    }
}
#[derive(Debug, Copy, Clone,PartialEq)]
pub struct Position{
x:i32,
y:i32,
}
impl Position {
    pub fn new(x:i32,y:i32) -> Self {
         Self{ x:x,
               y:y}
    }
    pub fn render(&self,canvas:&mut sdl2::render::WindowCanvas)  {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(50,250,250));
        canvas.draw_rect(Rect::new(self.x*120,self.y*120,120,120));
        for i in 1..10{
            canvas.draw_rect(Rect::new(self.x*120,self.y*120,120-i,120-i));
        }
    }
}

#[derive(Debug, Copy, Clone,PartialEq)]
pub struct Piece{
    type_:u32,
    color:u32,
    x:i32,
    y:i32,
    value:u32,
/*
 * type 0 = pawn
 * type 1 = rook
 * type 2 = horse
 * type 3 = bishop
 * type 4 = queen
 * type 5 = king
 */ 
}
impl Piece{
     pub fn new(t:u32,c:u32,x:i32,y:i32,value:u32) -> Self {
            if c>1{
                println!("Piece initialization error color must be either 1 or 0");
            }
            Self{ 
                type_:t,
                color:c,
                x:x,
                y:y,
                value:value,
            }

    }
    pub fn movePiece(&mut self,tx:i32,ty:i32){
        self.x=tx;
        self.y=ty;
    }
    pub fn render(&self,canvas:&mut sdl2::render::WindowCanvas,texture: &Texture)  {
        let w=80;
        let h=80;
        canvas.copy(texture,Rect::new(0,0,100,100),Rect::new(self.x*120+25,self.y*120+25,w,h));
    }
    pub fn print(&self){
        match self.type_{
            0 =>print!("pawn "),
            1 =>print!("rook "),
            2 =>print!("horse "),
            3 =>print!("bishop"),
            4 =>print!("queen "),
            5 =>print!("king "),
            _=>print!("type error"),
        }
        println!("color {} x {} y {} value {}",self.color,self.x,self.y,self.value);
    }
    //moves
    // i need to check for collisions with same color pieces later
    //to be returned with match _type
    pub fn pawn_moves(&mut self,pieces:Vec<Piece>)->Vec<Position>{
        let mut moves = Vec::<Position>::new();
        moves.push(Position::new(self.x,self.y));
        if self.color == 1{
            if self.y >0 {
            let mut collision1=false;
            for piece in &pieces{
                collision1=piece.x==self.x&&piece.y==self.y-1;
                if collision1{
                    break;
                }
                //println!("{:?} ",collision1);
            }
            if !collision1{
                moves.push(Position::new(self.x,self.y-1));}
            }
            if self.y == 6{
            let mut collision2=false;
            for piece in &pieces{
                collision2=piece.x==self.x&&piece.y==self.y-2;
                if collision2{
                    break;
                }
                //println!("{:?} ",collision2);
            }
            if !collision2{
                moves.push(Position::new(self.x,self.y-2));
            }
            }
            for piece in &pieces{
                if piece.x==self.x-1 && self.y ==piece.y+1 &&piece.color==0 {
                    moves.push(Position::new(self.x-1,self.y-1));
                }
                if piece.x==self.x+1 && self.y ==piece.y+1 &&piece.color==0{
                    moves.push(Position::new(self.x+1,self.y-1));
                }
            }
        }
        if self.color == 0 {
            
            if self.y >0 {
            let mut collision1=false;
            for piece in &pieces{
                collision1=piece.x==self.x&&piece.y==self.y+1;
                if collision1{
                    break;
                }
                //println!("{:?} ",collision1);
            }
            if !collision1{
                moves.push(Position::new(self.x,self.y+1));}
            }
            if self.y == 1{
            let mut collision2=false;
            for piece in &pieces{
                collision2=piece.x==self.x&&piece.y==self.y+2;
                if collision2{
                    break;
                }
                //println!("{:?} ",collision2);
            }
            if !collision2{
                moves.push(Position::new(self.x,self.y+2));
            }
            }
            for piece in &pieces{
                if piece.x==self.x+1 && self.y ==piece.y-1 &&piece.color==1 {
                    moves.push(Position::new(self.x+1,self.y+1));
                }
                if piece.x==self.x-1 && self.y ==piece.y-1 &&piece.color==1{
                    moves.push(Position::new(self.x-1,self.y+1));
                }
            }
        }
        return moves;
    }
    pub fn horse_moves(&mut self,pieces:Vec<Piece>)->Vec<Position>{
        let mut moves = Vec::<Position>::new();
        moves.push(Position::new(self.x,self.y));
        let x=self.x;
        let y=self.y;
        if x-2>=0{
            if y-1>=0
            {   
                let mut collision=false;
                for piece in &pieces{
                    if piece.color == self.color && piece.x == self.x-2 && piece.y==self.y-1{
                        collision=true;
                    }
                    if collision{
                        break;
                    }
                }
                if !collision{
                moves.push(Position::new(self.x-2,self.y-1));}
                

            }
            if y+1<8
            {    
                let mut collision=false;
                for piece in &pieces{
                    if piece.color == self.color && piece.x == self.x-2 && piece.y==self.y+1{
                        collision=true;
                    }
                    if collision{
                        break;
                    }
                }
                if !collision{
                moves.push(Position::new(self.x-2,self.y+1));}
            }
        }
        if x+2<8{
            if y-1>=0
            {  
                let mut collision=false;
                for piece in &pieces{
                    if piece.color == self.color && piece.x == self.x+2 && piece.y==self.y-1{
                        collision=true;
                    }
                    if collision{
                        break;
                    }
                }
                if !collision{
                moves.push(Position::new(self.x+2,self.y-1));}
            }
            if y+1<8
            { 
                let mut collision=false;
                for piece in &pieces{
                    if piece.color == self.color && piece.x == self.x+2 && piece.y==self.y+1{
                        collision=true;
                    }
                    if collision{
                        break;
                    }
                }
                if !collision{
                moves.push(Position::new(self.x+2,self.y+1));}
            }
        }
        if y-2>=0{
            if x+1<8
            {   
                let mut collision=false;
                for piece in &pieces{
                    if piece.color == self.color && piece.x == self.x+1 && piece.y==self.y-2{
                        collision=true;
                    }
                    if collision{
                        break;
                    }
                }
                if !collision{
                moves.push(Position::new(self.x+1,self.y-2));}
            }
            if x-1>=0
            {   
                let mut collision=false;
                for piece in &pieces{
                    if piece.color == self.color && piece.x == self.x-1 && piece.y==self.y-2{
                        collision=true;
                    }
                    if collision{
                        break;
                    }
                }
                if !collision{
                moves.push(Position::new(self.x-1,self.y-2));}
            }
        }
        if y+2<8{
            if x-1>=0
            {   
                let mut collision=false;
                for piece in &pieces{
                    if piece.color == self.color && piece.x == self.x-1 && piece.y==self.y+2{
                        collision=true;
                    }
                    if collision{
                        break;
                    }
                }
                if !collision{
                moves.push(Position::new(self.x-1,self.y+2));}}
            if x+1<8
            {   
                let mut collision=false;
                for piece in &pieces{
                    if piece.color == self.color && piece.x == self.x+1 && piece.y==self.y+2{
                        collision=true;
                    }
                    if collision{
                        break;
                    }
                }
                if !collision{
                moves.push(Position::new(self.x+1,self.y+2));}}
        }
        return moves;
}

    pub fn rook_moves(&mut self,pieces:Vec<Piece>) -> Vec<Position> {
        let mut moves = Vec::<Position>::new();
        moves.push(Position::new(self.x,self.y));
        let x=self.x;
        let y=self.y;
        for i in (x+1)..8{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == i && piece.y==self.y{
                    collision=true;
                    if piece.color != self.color{
                        moves.push(Position::new(i,self.y));
                    }
                }
                if collision{
                    break;
                }
            }
            if collision{
                break;
            }
            moves.push(Position::new(i,self.y));
        }
        for i in (y+1)..8{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x && piece.y==i{
                    collision=true;
                    if piece.color != self.color{
                        moves.push(Position::new(self.x,i));
                    }
                }
                if collision{
                    break;
                }
            }
            if collision{
                break;
            }
            moves.push(Position::new(self.x,i));
        }
        for i in (0..x).rev(){
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == i && piece.y==self.y{
                    collision=true;
                    if piece.color != self.color{
                        moves.push(Position::new(i,self.y));
                    }
                }
                if collision{
                    break;
                }
            }
            if collision{
                break;
            }
            moves.push(Position::new(i,self.y));
        }
        for i in (0..y).rev(){
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x && piece.y==i{
                    collision=true;
                    if piece.color != self.color{
                        moves.push(Position::new(self.x,i));
                    }
                }
                if collision{
                    break;
                }
            }
            if collision{
                break;
            }
            moves.push(Position::new(self.x,i));
        }
        return moves;

    }
    pub fn bishop_moves(&mut self,pieces:Vec<Piece>) -> Vec<Position> {

        let mut moves = Vec::<Position>::new();
        moves.push(Position::new(self.x,self.y));
        let x=self.x;
        let y=self.y;
        for i in 1..16{
            if x+i<8 && x+i>=0 && y+i<8 && y+i>=0{
            
            let mut collision=false;
                for piece in &pieces{
                    if  piece.x == self.x+i && piece.y==self.y+i{
                        if piece.color != self.color{
                            moves.push(Position::new(x+i,y+i));
                        }
                        collision=true;
                    }
                }
            if collision{
                break;
            }
            moves.push(Position::new(x+i,y+i));
            }
        }
        for i in 1..16{
            if x-i<8 && x-i>=0 && y+i<8 && y+i>=0{
            let mut collision=false;
                for piece in &pieces{
                    if  piece.x == self.x-i && piece.y==self.y+i{
                        if piece.color != self.color{
                            moves.push(Position::new(x-i,y+i));
                        }
                        collision=true;
                    }
                }
            if collision{
                break;
            }
            moves.push(Position::new(x-i,y+i));
            }
        }
        for i in 1..16{
            if x+i<8 && x+i>=0 && y-i<8 && y-i>=0{
            let mut collision=false;
                for piece in &pieces{
                    if  piece.x == self.x+i && piece.y==self.y-i{
                        if piece.color != self.color{
                            moves.push(Position::new(x+i,y-i));
                        }
                        collision=true;
                    }
                }
            if collision{
                break;
            }
            moves.push(Position::new(x+i,y-i));
            }
        }
        for i in 1..16{
            if x-i<8 && x-i>=0 && y-i<8 && y-i>=0{
            let mut collision=false;
                for piece in &pieces{
                    if  piece.x == self.x-i && piece.y==self.y-i{
                        if piece.color != self.color{
                            moves.push(Position::new(x-i,y-i));
                        }
                        collision=true;
                    }
                }
            if collision{
                break;
            }
            moves.push(Position::new(x-i,y-i));
            }
        }
        return moves;

    }
    pub fn queen_moves(&mut self,pieces:Vec<Piece>) -> Vec<Position> {

        let mut moves = Vec::<Position>::new();
        moves.push(Position::new(self.x,self.y));
        let x=self.x;
        let y=self.y;
        moves.append(&mut self.bishop_moves(pieces.to_vec()));
        moves.append(&mut self.rook_moves(pieces.to_vec()));
        return moves;
    }
    pub fn king_moves(&mut self,pieces:Vec<Piece>) -> Vec<Position> {

        let mut moves = Vec::<Position>::new();
        moves.push(Position::new(self.x,self.y));
        let x=self.x;
        let y=self.y;
            if x-1>=0{
                let mut collision=false;
                for piece in &pieces{
                    if  piece.x == self.x-1 && piece.y==self.y{
                        if piece.color != self.color{
                            moves.push(Position::new(x-1,y));
                        }
                        collision=true;
                        if collision{
                        break;}
                    }
                }
            if !collision{
            moves.push(Position::new(self.x-1,self.y));}
            if y-1 >=0{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x-1 && piece.y==self.y-1{
                    if piece.color != self.color{
                        moves.push(Position::new(x-1,y-1));
                    }
                    collision=true;
                    if collision{
                    break;}

                }
            }
            if !collision{
            moves.push(Position::new(self.x-1,self.y-1));}
            }
            if y+1 <8{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x-1 && piece.y==self.y+1{
                    if piece.color != self.color{
                        moves.push(Position::new(x-1,y+1));
                    }
                    collision=true;
                    if collision{
                    break;}
                }
            }
            if !collision{
            moves.push(Position::new(self.x-1,self.y+1));}}
            }
            if y+1 <8{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x && piece.y==self.y+1{
                    if piece.color != self.color{
                        moves.push(Position::new(x,y+1));
                    }
                    collision=true;
                    if collision{
                    break;}
                }
            }
            if !collision{
            moves.push(Position::new(self.x,self.y+1));}}
            if y-1 >=0{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x && piece.y==self.y-1{
                    if piece.color != self.color{
                        moves.push(Position::new(x,y-1));
                    }
                    collision=true;
                    if collision{
                    break;}
                }
            }
            if !collision{
            moves.push(Position::new(self.x,self.y-1));}}
            if x+1 <8{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x+1 && piece.y==self.y{
                    if piece.color != self.color{
                        moves.push(Position::new(x+1,y));
                    }
                    collision=true;
                    if collision{
                    break;}
                }
            }
            if !collision{
            moves.push(Position::new(self.x+1,self.y));}
            if y+1 <8{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x+1 && piece.y==self.y+1{
                    if piece.color != self.color{
                        moves.push(Position::new(x+1,y+1));
                    }
                    collision=true;
                    if collision{
                    break;}
                }
            }if !collision{
            moves.push(Position::new(self.x+1,self.y+1));}}
            if y-1 >=0{
            let mut collision=false;
            for piece in &pieces{
                if  piece.x == self.x+1 && piece.y==self.y-1{
                    if piece.color != self.color{
                        moves.push(Position::new(x+1,y-1));
                    }
                    collision=true;
                    if collision{
                    break;}
                }
            }if !collision{
            moves.push(Position::new(self.x+1,self.y-1));}}
            }
        return moves;
    }


}
pub struct Board{
   color1:sdl2::pixels::Color, 
   color2:sdl2::pixels::Color, 
}
impl Board {
   pub fn new(c1:sdl2::pixels::Color,c2:sdl2::pixels::Color) -> Self {
           Self{
            color1:c1,
            color2:c2,
           }
      }
   pub fn render(&self,canvas:&mut  sdl2::render::WindowCanvas){
       let mut change=0;
       let w=120;
       for i in 0..8{
            if change==0{
                change=1;
                canvas.set_draw_color(self.color1);
            }else{
                change=0;
                canvas.set_draw_color(self.color2);
            }
            for j in 0..8{
                canvas.fill_rect(Rect::new((i*w)as i32,(j*w) as i32,w,w)).unwrap();
                if change==0{
                    change=1;
                    canvas.set_draw_color(self.color1);
                }else{
                    change=0;
                    canvas.set_draw_color(self.color2);

                }
            }
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
   }
}
pub struct Game{
    board:Board,
    pieces:Vec<Piece>,
    pub selected_piece:Selected_Piece,
}
impl Game{
    pub fn new() -> Self {    
        Self{ 
                board:Board::new(sdl2::pixels::Color::RGB(0,0,255),sdl2::pixels::Color::RGB(255,255,255)),
                pieces:Vec::<Piece>::new(),
                selected_piece:Selected_Piece::empty_new(),}
            
       }
    pub fn putPieces(&mut self){
        self.pieces = Vec::<Piece>::new();
        for i in 0..8{
            self.pieces.push(Piece::new(0,0,i,1,1));
            self.pieces.push(Piece::new(0,1,i,6,1));
        }
            self.pieces.push(Piece::new(1,0,0,0,5));
            self.pieces.push(Piece::new(1,1,0,7,5));
            self.pieces.push(Piece::new(2,0,1,0,3));
            self.pieces.push(Piece::new(2,1,1,7,3));
            self.pieces.push(Piece::new(3,0,2,0,3));
            self.pieces.push(Piece::new(3,1,2,7,3));
            self.pieces.push(Piece::new(4,0,4,0,8));
            self.pieces.push(Piece::new(4,1,4,7,8));
            self.pieces.push(Piece::new(5,0,3,0,10));
            self.pieces.push(Piece::new(5,1,3,7,10));
            self.pieces.push(Piece::new(1,0,7,0,5));
            self.pieces.push(Piece::new(1,1,7,7,5));
            self.pieces.push(Piece::new(2,0,6,0,3));
            self.pieces.push(Piece::new(2,1,6,7,3));
            self.pieces.push(Piece::new(3,0,5,0,3));
            self.pieces.push(Piece::new(3,1,5,7,3));

    }
    pub fn get_piece_by_clicking(&mut self,x:i32,y:i32,turn:u32)->Option<&mut Piece>{
        //collision checking
        let mut pieces2=Vec::<Piece>::new();
        pieces2= self.pieces.to_vec();
        //
        let x_coor:i32=x/120;
        let y_coor:i32=y/120;
        for piece in &mut self.pieces{
            if piece.x==x_coor && piece.y == y_coor && piece.color==turn%2{
                println!("{}" ,turn);
                self.selected_piece.selected=true;
                match piece.type_{ 
                    0=>self.selected_piece.moves=piece.pawn_moves(pieces2),
                    1=>self.selected_piece.moves=piece.rook_moves(pieces2),
                    2=>self.selected_piece.moves=piece.horse_moves(pieces2),
                    3=>self.selected_piece.moves=piece.bishop_moves(pieces2),
                    4=>self.selected_piece.moves=piece.queen_moves(pieces2),
                    5=>self.selected_piece.moves=piece.king_moves(pieces2),
                    _=>print!("wrong type or not implemented yet"),


                }
                return Some(piece);
            }
        }
        None
    }
    // t stand for target
    // x y are the piece coordinates
    pub fn movePiece(&mut self,tx:i32,ty:i32)->u32{

        let x_coor:i32=self.selected_piece.moves[0].x;
        let y_coor:i32=self.selected_piece.moves[0].y;
        let mut index;
        index = self.pieces.iter().position(|x| x.x==tx/120 && x.y==ty/120 );
        if tx/120 != x_coor && ty/120 != y_coor{
        if !index.is_none(){
        self.pieces.remove(index.unwrap());}
        }

        if tx/120 == x_coor && ty/120 == y_coor{
            self.selected_piece.selected=false;
           return 0; 
        }
        for piece in &mut self.pieces{
            if piece.x==x_coor && piece.y == y_coor {
                if self.selected_piece.moves.contains(&Position::new(tx/120,ty/120)){
                    piece.movePiece(tx/120,ty/120);
                    self.selected_piece.selected=false;
                    return 1;
                }
            }
        }
        self.selected_piece.selected=false;
        return 0;
    }
    pub fn renderComponents(& self,canvas:&mut sdl2::render::WindowCanvas,textures:&[Texture;12]){
        self.board.render(canvas);
        if self.selected_piece.selected{
            for position in &self.selected_piece.moves{
                position.render(canvas)
            }
        }
        for piece in &self.pieces{
            piece.render(canvas,&textures[(piece.type_+6*piece.color) as usize]);
        }
    }
}


