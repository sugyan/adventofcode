type answer = Integer of int

module type Solve = sig
  val solve : in_channel -> answer * answer
end
