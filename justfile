test day:
  cd day-{{day}} && cargo test  

create day:
  cargo new day-{{day}}
  aoc download -d {{day}} -i day-{{day}}/input -p day-{{day}}/README.md
  cp template/src/main.rs day-{{day}}/src/main.rs

publish day:
  aoc download -d {{day}} -p day-{{day}}/README.md -o
  # format the README.md for github
  git add day-{{day}}
  git commit -m "added day {{day}}"
  git push github master
