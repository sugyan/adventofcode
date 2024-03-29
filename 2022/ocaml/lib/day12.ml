open Base
open Solution

module Solution : Solve = struct
  type t = (char -> bool) -> int

  let parse input =
    let grid =
      Stdio.In_channel.input_lines input
      |> List.map ~f:String.to_array
      |> List.to_array
    in
    fun finish ->
      let r, c = (Array.length grid, Array.length grid.(0)) in
      let p =
        List.cartesian_product (List.range 0 r) (List.range 0 c)
        |> List.find_exn ~f:(fun (i, j) -> Char.(grid.(i).(j) = 'E'))
      in
      let height = function
        | 'S' -> Char.to_int 'a'
        | 'E' -> Char.to_int 'z'
        | c -> Char.to_int c
      in
      let mins = Array.make_matrix ~dimx:r ~dimy:c None in
      let q = Queue.create () in
      let rec bfs ((i, j), d) =
        if finish grid.(i).(j) then d
        else
          let h = height grid.(i).(j) in
          let moveble (i, j) =
            (i >= 0 && i < r && j >= 0 && j < c)
            && Option.is_none mins.(i).(j)
            && height grid.(i).(j) >= h - 1
          in
          let enqueue (i, j) =
            mins.(i).(j) <- Some (d + 1);
            Queue.enqueue q ((i, j), d + 1)
          in
          [ (i - 1, j); (i + 1, j); (i, j - 1); (i, j + 1) ]
          |> List.filter ~f:moveble |> List.iter ~f:enqueue;
          bfs (Queue.dequeue_exn q)
      in
      bfs (p, 0)

  let part1 min_steps =
    min_steps (function 'S' -> true | _ -> false) |> answer_of_int

  let part2 min_steps =
    min_steps (function 'S' | 'a' -> true | _ -> false) |> answer_of_int
end
