use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(sum);

    my %OPS = (
        add           => sub { $_[0] + $_[1] },
        multiply      => sub { $_[0] * $_[1] },
        concatenation => sub { $_[0] . $_[1] },
    );

    method parse($lines) {
        return [
            map {
                my ( $test_value, $numbers ) = split /:\s+/, $_, 2;
                {
                    test_value => 0 + $test_value,
                    numbers    => [ split /\s+/, $numbers ],
                }
            } $lines->@*
        ];
    }

    method part1() {
        my $input = $self->input();
        my $ops   = [ map { $OPS{$_} } qw(add multiply) ];
        return sum map { $_->{test_value} }
          grep { is_possible( $ops, $_->{test_value}, $_->{numbers}->@* ) }
          $input->@*;
    }

    method part2() {
        my $input = $self->input();
        my $ops   = [ map { $OPS{$_} } qw(add multiply concatenation) ];
        return sum map { $_->{test_value} }
          grep { is_possible( $ops, $_->{test_value}, $_->{numbers}->@* ) }
          $input->@*;
    }

    sub is_possible ( $ops, $target, $current, @numbers ) {
        return $target == $current unless @numbers;
        return 0 if $current > $target;
        my ( $n, @rest ) = @numbers;
        for my $op ( $ops->@* ) {
            return 1
              if is_possible( $ops, $target, $op->( $current, $n ), @rest );
        }
        return 0;
    }
}
