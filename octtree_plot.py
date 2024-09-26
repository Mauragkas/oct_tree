import sys
import json
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d.art3d import Poly3DCollection

POINTS = []

# Function to plot a single octree node
def plot_octree(node, ax):
    if not node:
        return

    # Get the boundary of the current node
    center = node['boundary']['center']
    half_size = node['boundary']['half_size']

    # Plot the boundary as a 3D box
    box_points = [
        [center['x'] - half_size, center['y'] - half_size, center['z'] - half_size],  # bottom-back-left
        [center['x'] + half_size, center['y'] - half_size, center['z'] - half_size],  # bottom-back-right
        [center['x'] + half_size, center['y'] + half_size, center['z'] - half_size],  # bottom-front-right
        [center['x'] - half_size, center['y'] + half_size, center['z'] - half_size],  # bottom-front-left
        [center['x'] - half_size, center['y'] - half_size, center['z'] + half_size],  # top-back-left
        [center['x'] + half_size, center['y'] - half_size, center['z'] + half_size],  # top-back-right
        [center['x'] + half_size, center['y'] + half_size, center['z'] + half_size],  # top-front-right
        [center['x'] - half_size, center['y'] + half_size, center['z'] + half_size],  # top-front-left
    ]

    # Connect points to form the faces of the box
    box_faces = [
        [box_points[0], box_points[1], box_points[2], box_points[3]],  # bottom face
        [box_points[4], box_points[5], box_points[6], box_points[7]],  # top face
        [box_points[0], box_points[1], box_points[5], box_points[4]],  # back face
        [box_points[2], box_points[3], box_points[7], box_points[6]],  # front face
        [box_points[0], box_points[3], box_points[7], box_points[4]],  # left face
        [box_points[1], box_points[2], box_points[6], box_points[5]],  # right face
    ]

    # Plot the box using Poly3DCollection
    ax.add_collection3d(Poly3DCollection(box_faces, linewidths=0.5, edgecolors='b', alpha=0.0))

    # Plot the points at the current node
    for point in node['points']:
        POINTS.append(point)
        ax.scatter(point['x'], point['y'], point['z'], c='r', marker='o', s=2)  # Red dots for points

    # Recursively plot the eight children if the node is subdivided
    if node['divided']:
        plot_octree(node['fne'], ax)  # north-east (front-top-right)
        plot_octree(node['fnw'], ax)  # north-west (front-top-left)
        plot_octree(node['fse'], ax)  # south-east (front-bottom-right)
        plot_octree(node['fsw'], ax)  # south-west (front-bottom-left)
        plot_octree(node['bne'], ax)  # north-east-back (back-top-right)
        plot_octree(node['bnw'], ax)  # north-west-back (back-top-left)
        plot_octree(node['bse'], ax)  # south-east-back (back-bottom-right)
        plot_octree(node['bsw'], ax)  # south-west-back (back-bottom-left)

# Main function to read JSON and plot octree
def plot_octree_from_json(json_file, limit=800):
    # Read the JSON data
    with open(json_file, 'r') as f:
        data = json.load(f)

    # Create a 3D plot
    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')

    # Set the background color (RGB 255, 255, 237)
    fig.patch.set_facecolor((1.0, 1.0, 0.929))
    ax.set_facecolor((1.0, 1.0, 0.929))

    # Set plot limits (adjust these if necessary)
    ax.set_xlim(-limit, limit)
    ax.set_ylim(-limit, limit)
    ax.set_zlim(-limit, limit)

    # Plot the octree
    plot_octree(data, ax)

    plt.savefig('octree_plot.png', dpi=1200)

    # Show the plot
    plt.show()

# Example usage:
# read execution arguments
if __name__ == "__main__":
    try:
        plot_octree_from_json(sys.argv[1], int(sys.argv[2]))
    except KeyboardInterrupt:
        print("Interrupted by user")
        plt.close('all')  # Close all matplotlib windows
    except Exception as e:
        print("Error: ", e)
    finally:
        print("Number of points: ", len(POINTS))
