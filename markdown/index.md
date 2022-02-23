### practica
## tema: Game developemnt in Rust porgramming language

## ideas:
- using 3d graphics
- character camera, movement
- world exploring without any serious game mecanics
    - day / night cycle
- focus on world generation terrain, plants ( possible even creatures )
    - solar system exploring
    - noise algorithms for terrain
        - possible sphere shaped world with multiple planets, satellites.
    - L-systems or parametric palnt generation
    - find more about living organisms....
- integrating **lua** scripts in this game
    - or an alternative more apropriate scriptin language but lua is lightweigh
      (300 KB)
    - alternative would be wo write my own script parser ( compiler ) 
- studying shaders (programming language for gpu)
    - this probably will require a lot of math -_-

## pasi de implementare:
- crarea unei harti simple ( fara geometrie legata de inaltime ) doar un plan
- implementarea posibilitatii de a misca camera
    - posibil de creat un caracter si de facut switch intre 1st/3rd person
- experimentarea cu diferiti algoritmi de generare a terenului si posibilitatea
  de a folosi multipli in jocul dat din setari
- generarea de plante si distribuirea lor tot utilizand algoritmii de mai sus
- implementarea unei interfete grafice ( meniu, setari, savegame )
- conectarea unui limbaj interpretat si crearea unui API care poate fi accesat
  din acest script

saptamani:
1. Cercetarea modului de creare a jocurilor.
2. Familiarizarea cu motorul de jocuri "Bevy" si paradigma ECS.
3. Crearea unui scene simple si realizarea controlului camerei.
4. Generarea procedurala a terenului scenei.
5. Generarea procedurala de plante.
6. Implementarea unei interfete grafice.
7. Popularea lumii cu NPC.
8. Integrarea Lua in codul jocului. Inbunatatirea jocului.
