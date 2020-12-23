#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int x) : val(x), next(nullptr) {
    }
};

unsigned long long solve(vector<int>& v, int num_cups, int num_moves) {
    unordered_map<int, ListNode*> um;

    ListNode* prev = nullptr;
    for (int& n : v) {
        ListNode* node = new ListNode(n);
        um[n] = node;
        if (prev) {
            prev->next = node;
        }
        prev = node;
    }
    for (int i = v.size(); i < num_cups; ++i) {
        ListNode* node = new ListNode(i + 1);
        um[i + 1] = node;
        if (prev) {
            prev->next = node;
        }
        prev = node;
    }
    if (prev) {
        prev->next = um[v[0]];
    }

    int pickup[3];
    ListNode* current = um[v[0]];
    for (int i = 0; i < num_moves; ++i) {
        ListNode* node = current;
        for (int i = 0; i < 3; ++i) {
            node = node->next;
            pickup[i] = node->val;
        }
        int destination = current->val > 1 ? current->val - 1 : num_cups;
        while (pickup[0] == destination || pickup[1] == destination ||
               pickup[2] == destination) {
            destination = destination > 1 ? destination - 1 : num_cups;
        }
        ListNode* tmp = node->next;
        node->next = um[destination]->next;
        um[destination]->next = current->next;
        current->next = tmp;
        current = current->next;
    }
    return 1ull * um[1]->next->val * um[1]->next->next->val;
}
int main() {
    string s;
    cin >> s;
    vector<int> v;
    for (char& c : s) {
        v.push_back(c - '0');
    }
    cout << solve(v, 1'000'000, 10'000'000) << endl;
}
