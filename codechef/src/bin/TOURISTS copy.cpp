#include <bits/stdc++.h>
using namespace std;

using edge = pair<int, int>;

int n, e;
vector<edge> roads;
vector<deque<edge>> gph;
vector<bool> vis;
set<edge> ans;
vector<int> path;

void dfs(int node)
{
    while (!gph[node].empty())
    {
        auto v = gph[node].front();
        gph[node].pop_front();
        if (!vis[v.first])
        {
            vis[v.first] = true;
            if (path.empty())
            {
                path.emplace_back(node);
            }
            path.emplace_back(v.second);
            dfs(v.second);
        }
    }
    if (!path.empty())
    {
        for (int i = 1; i < path.size(); ++i)
        {
            ans.emplace(make_pair(path[i - 1], path[i]));
        }
        path.clear();
    }
}

int main()
{
    cin.tie(0), cout.tie(0), ios_base::sync_with_stdio(false);

    cin >> n >> e;
    gph.resize(n + 1);
    vis.resize(e + 1, false);

    int u, v;
    for (int i = 0; i < e; ++i)
    {
        cin >> u >> v;
        gph[u].emplace_back(make_pair(i, v));
        gph[v].emplace_back(make_pair(i, u));
        roads.emplace_back(make_pair(u, v));
    }

    if (e < n)
    {
        cout << "NO\n";
        return 0;
    }

    for (auto &g : gph)
    {
        if (g.size() % 2 != 0)
        {
            cout << "NO\n";
            return 0;
        }
    }

    dfs(1);
    if (ans.size() < roads.size())
    {
        cout << "NO\n";
        return 0;
    }

    cout << "YES\n";
    for (auto road : roads)
    {
        if (ans.find(road) != ans.end())
        {
            cout << road.first << ' ' << road.second << '\n';
        }
        else
        {
            cout << road.second << ' ' << road.first << '\n';
        }
    }

    return 0;
}
