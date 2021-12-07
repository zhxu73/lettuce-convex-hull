#!/usr/bin/julia

using Plots
using PlyIO

function plot_ply_2d(filename)
    ply = load_ply(filename)
    graph = Plots.scatter(ply["vertex"]["x"], ply["vertex"]["y"], title=filename)
    graph
end

function plot_ply_3d(filename)
    ply = load_ply(filename)
    graph = Plots.scatter(ply["vertex"]["x"], ply["vertex"]["y"], ply["vertex"]["z"], title=filename)
    graph
end

function convex_hull_2d()
    filename = "output.ply"
    plot_ply_2d(filename)
end

function raw_data()
    filename = "data/lettuce.ply"
    plot_ply_3d(filename)
end

function data_soil_removed()
    filename = "soil_removed.ply"
    plot_ply_3d(filename)
end

function data_3d()
    filename = "output.ply"
    plot_ply_3d(filename)
end

function data_soil_removed_o3d()
    filename = "soil_removed_o3d.ply"
    plot_ply_3d(filename)
end

# graph = raw_data()
graph = data_soil_removed()
# graph = data_soil_removed_o3d()
# graph = convex_hull_2d()
# graph = data_3d()
display(graph)
