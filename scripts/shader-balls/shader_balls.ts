class Vector3 {
    x: number;
    y: number;
    z: number;

    constructor(x: number = 0, y: number = 0, z: number = 0) {
        this.x = x;
        this.y = y;
        this.z = z;
    }
}

class Transform {
    position: Vector3;
    rotation: Vector3;

    constructor(position: Vector3 = new Vector3(), rotation: Vector3 = new Vector3()) {
        this.position = position;
        this.rotation = rotation;
    }
}

class RenderObject {
    texture_id: number;
    mesh_id: number;
    transform: Transform;

    constructor(texture_id: number = null, mesh_id: number = null, transform: Transform = new Transform()) {
        this.texture_id = texture_id;
        this.mesh_id = mesh_id;
        this.transform = transform;
    }
}

class Mesh {

};

class FloatMatrix {
    buffer: Float64Array;
    m: number;
    n: number;

    constructor(buffer: Float64Array, m: number, n: number) {
        this.m = m;
        this.n = n;
        this.buffer = buffer;
    }

    multiply(matrix_b: FloatMatrix): FloatMatrix {
        
        let dot_product = new FloatMatrix(new Float64Array(this.m * matrix_b.n), this.m, matrix_b.n);
        for (let a_row = 0; a_row < this.m; a_row++) {
            for (let b_col = 0; b_col < matrix_b.n; b_col++) {
                let product_sum = 0;
                for (let a_col = 0; a_col < this.n; a_col++) {
                    for (let b_row = 0; b_row < matrix_b.m; b_row++) {
                        product_sum += this.buffer[a_row * this.n + a_col] * matrix_b.buffer[b_row * matrix_b.n + b_col];
                    }
                }
                dot_product.buffer[a_row * dot_product.n + b_col] = product_sum;
            }
        }
        return dot_product ;
    }
};

var render_objects: Array<RenderObject> = new Array();
var texture_map: Map<number, WebGLTexture> = new Map();
var mesh_map: Map<number, Mesh> = new Map();
function render() {
    requestAnimationFrame(render);
    
}

function start() {
    render();
}

function create_sphere(radius_meters: number = 1, segments: number = 6, rings: number = 12) {
    let curve_vertices: Array<Vector3> = [];
    
    for(let i = 0; i < rings; i++) {
        let theta = Math.PI / 2 - (Math.PI / rings * i);
        let x = Math.cos(theta) * radius_meters;
        let y = Math.sin(theta) * radius_meters;

        curve_vertices.push(new Vector3(x, y));
    }

    let rotation_matrix: FloatMatrix = new FloatMatrix(new Float64Array(), 2, 2);

    let sphere_vertices: Array<Vector3> = [];

    

    return new RenderObject();
    
}

let A = new FloatMatrix(new Float64Array([1, 2, 3, 4]), 2, 2);
let B = new FloatMatrix(new Float64Array([5, 6, 7, 8]), 2, 2);

console.log(A.multiply(B));
