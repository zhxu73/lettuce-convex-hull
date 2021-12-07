import argparse
import open3d as o3d
import numpy as np
import os

def main():
    parser = argparse.ArgumentParser(description="visualize ply file using open3d.")
    parser.add_argument("filename", nargs=1, help="path to the ply file")
    args = parser.parse_args()
    if isinstance(args.filename, list):
        ply_path = args.filename[0]
    else:
        ply_path = args.filename
    visualize(ply_path)

# modified from
# https://github.com/phytooracle/Lettuce_Soil_Annotated_PointCloud_Visualizer/blob/master/utils.py
def visualize(ply_path: str):
    pcd = o3d.io.read_point_cloud(ply_path)

    points = np.array(pcd.points)
    mean = points.mean(axis=0)
    points = points - mean
    points /= np.linalg.norm(points, axis=1).max()

    pcd.points = o3d.utility.Vector3dVector(points)

    o3d.visualization.draw_geometries([pcd])



if __name__ == '__main__':
    main()

