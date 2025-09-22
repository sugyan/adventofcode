use 5.38.0;
use feature 'class';
no warnings 'experimental::class';

use Base;

class Solution : isa(Base) {
    use List::Util qw(all any);

    method parse($lines) {
        my @reports = map { [ split /\s+/, $_ ] } $lines->@*;
        return \@reports;
    }

    method part1() {
        my $input = $self->input();
        return scalar grep { is_safe($_) } $input->@*;
    }

    method part2() {
        my $input = $self->input();
        return scalar grep {
            my $report = $_;
            any {
                my @copied = @$report;
                splice( @copied, $_, 1 );
                is_safe( \@copied )
            } 0 .. $#$report
        } $input->@*;
    }

    sub is_safe ($report) {
        my @diffs = map { $report->[$_] - $report->[ $_ - 1 ] } 1 .. $#$report;
        my $increasing = all { $_ > 0 } @diffs;
        my $decreasing = all { $_ < 0 } @diffs;
        return ( $increasing || $decreasing )
          && all { my $d = abs($_); $d >= 1 && $d <= 3 } @diffs;
    }
}
