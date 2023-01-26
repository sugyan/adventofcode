open Stdio
open Base

let solve channel n =
  In_channel.input_lines channel
  |> List.group ~break:(fun x _ -> String.equal x "")
  |> List.map ~f:(fun l ->
         List.filter_map
           ~f:(function "" -> None | x -> Some (Int.of_string x))
           l
         |> List.fold ~init:0 ~f:( + ))
  |> List.sort ~compare:Int.compare
  |> List.rev
  |> (fun l -> List.take l n)
  |> List.fold ~init:0 ~f:( + )
;;

printf "Part1: %d\n" (solve Stdio.stdin 1)
(* printf "Part2: %d\n" (solve Stdio.stdin 3) *)
