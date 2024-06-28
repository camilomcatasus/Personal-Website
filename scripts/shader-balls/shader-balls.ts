function assert(condition: boolean, message: string | null = null) {
    if (!condition) {
        throw new Error(message || "Assertion failed");
    }
}

var vertex_shader_source = `#version 300 es
in vec3 a_position;

out vec3 vColor;

void main() {
    gl_Position = vec4(a_position, 1.0);
    vColor = vec3(a_position.x + 0.5, a_position.y + 0.5, a_position.z + 0.5); // Color based on position
}`;

var fragment_shader_source = `#version 300 es
precision mediump float;

in vec3 vColor;
out vec4 outColor;

uniform sampler2D uTexture;

void main() {
    outColor = vec4(vColor, 1.0);
}`;

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
    vertex_buffer: Float32Array;
    index_buffer: Uint16Array | null;
    index_buffer_used: boolean;

    constructor(vertex_buffer: Float32Array, index_buffer: Uint16Array | null = null, index_buffer_used: boolean = false) {
        this.vertex_buffer = vertex_buffer;
        this.index_buffer = index_buffer;
        this.index_buffer_used = index_buffer_used;
    }
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

    rotate_vector(column_vector: Vector3): Vector3 {
        assert(this.m == 3 && this.n == 3);
        let multiplied_column = new Vector3();

        multiplied_column.x += column_vector.x * this.buffer[0];
        multiplied_column.x += column_vector.y * this.buffer[1];
        multiplied_column.x += column_vector.z * this.buffer[2];

        multiplied_column.y += column_vector.x * this.buffer[3];
        multiplied_column.y += column_vector.y * this.buffer[4];
        multiplied_column.y += column_vector.z * this.buffer[5];

        multiplied_column.z += column_vector.x * this.buffer[6];
        multiplied_column.z += column_vector.y * this.buffer[7];
        multiplied_column.z += column_vector.z * this.buffer[8];

        return multiplied_column;
    }

    multiply(matrix_b: FloatMatrix): FloatMatrix {
        
        let dot_product = new FloatMatrix(new Float64Array(this.m * matrix_b.n), this.m, matrix_b.n);
        for (let a_row = 0; a_row < this.m; a_row++) {
            for (let b_col = 0; b_col < matrix_b.n; b_col++) {
                let product_sum = 0;
                let count = 0;
                while (count < this.n){
                    product_sum += this.buffer[a_row * this.n + count] * matrix_b.buffer[count * matrix_b.n + b_col];
                    console.log(`product_sum: ${product_sum}, a_row: ${a_row}, b_col: ${b_col}, count: ${count}`);
                    count += 1;
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


function create_y_axis_rotation_matrix(theta: number): FloatMatrix {
    return new FloatMatrix(new Float64Array([Math.cos(theta), 0, Math.sin(theta), 0, 1, 0, -Math.sin(theta), 0, Math.cos(theta)]), 3, 3);
}

function create_sphere_mesh(radius_meters: number = 1, segments: number = 6, rings: number = 12) {
    let sphere_vertices: Array<Vector3> = [new Vector3(0, 1, 0)];
    
    for(let i = 1; i < rings - 1; i++) {
        let theta = Math.PI / 2 - (Math.PI / rings * i);
        let x = Math.cos(theta) * radius_meters;
        let y = Math.sin(theta) * radius_meters;

        sphere_vertices.push(new Vector3(x, y));
        for (let j = 0; j < segments; j++) {
            let new_theta = Math.PI * 2 * j / segments;
            let rotation_matrix = create_y_axis_rotation_matrix(new_theta);

            sphere_vertices.push(rotation_matrix.rotate_vector(sphere_vertices[i]));
        }
    }

    let sphere_indices: Array<number> = [];

    for (let i = 0; i < segments - 1; i++) {
        sphere_indices.push(0);
        sphere_indices.push(i + 1);
        sphere_indices.push(i + 2);
    }

    for (let ring_index = 0; ring_index < rings - 2; ring_index ++) {
        for (let segment_index = 0; segment_index < segments - 1; segment_index++ ) {
            sphere_indices.push(ring_index * segments + 1 + segment_index);
            sphere_indices.push(ring_index * segments + 2 + segment_index);
            sphere_indices.push((ring_index + 1) * segments + segment_index * 2)

            sphere_indices.push(ring_index * segments + 1 + segment_index);
            sphere_indices.push((ring_index + 1) * segments + 1 + segment_index * 2)
            sphere_indices.push((ring_index + 1) * segments + 1 + (segment_index + 1) * 2)
        }
    }

    for (let i = 0; i < segments - 1; i++) {
        sphere_indices.push(sphere_vertices.length - 1);
        sphere_indices.push(sphere_vertices.length - 2 - i);
        sphere_indices.push(sphere_vertices.length - 3 - i);
    }

    let vertex_buffer = new Array<number>();
    for (let vertex_index = 0; vertex_index < sphere_vertices.length; vertex_index++) {
        vertex_buffer.push(sphere_vertices[vertex_index].x)
        vertex_buffer.push(sphere_vertices[vertex_index].y)
        vertex_buffer.push(sphere_vertices[vertex_index].z)
    }

    return new Mesh(new Float32Array(vertex_buffer), new Uint16Array(sphere_indices), true);
}

function create_rect_mesh(x: number, y: number, z: number, width: number, height: number): Mesh {
    return new Mesh(
        new Float32Array([
            // First tri
            x, y, z,
            x + width, y, z,
            x, y + height, z, 
            // Second tri
            x + width, y, z,
            x, y + height, z, 
            x + width, y + height, z, 
        ])
    )
}

let meshes = new Map<number, Mesh>();
const SPHERE_MESH = 0;
meshes.set(SPHERE_MESH, create_sphere_mesh(1, 5, 5))
const RECT_MESH = 1;
meshes.set(RECT_MESH, create_rect_mesh(0, 0, 0, 5, 5));

let current_mesh = meshes.get(SPHERE_MESH);

let canvas: HTMLCanvasElement = document.getElementsByTagName("canvas").item(0);
let gl = canvas.getContext("webgl2"); 
let objects: Array<RenderObject> = [new RenderObject(null, RECT_MESH, new Transform(new Vector3(0, 0, 0), new Vector3(0, 0, 0)))];

const vertex_shader = gl.createShader(gl.VERTEX_SHADER);
gl.shaderSource(vertex_shader, vertex_shader_source);
gl.compileShader(vertex_shader);

const fragment_shader = gl.createShader(gl.FRAGMENT_SHADER);
gl.shaderSource(fragment_shader, fragment_shader_source);
gl.compileShader(fragment_shader);

const shader_program = gl.createProgram();
gl.attachShader(shader_program, vertex_shader);
gl.attachShader(shader_program, fragment_shader);
gl.linkProgram(shader_program);

var positionAttributeLocation = gl.getAttribLocation(shader_program, "a_position");

gl.useProgram(shader_program);
gl.disable(gl.CULL_FACE);


function draw_mesh(gl: WebGL2RenderingContext, mesh: Mesh) {
    if (mesh.index_buffer_used) {

        const vao = gl.createVertexArray();
        gl.bindVertexArray(vao);

        gl.enableVertexAttribArray(positionAttributeLocation);

        const vertex_buffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);
        gl.bufferData(gl.ARRAY_BUFFER, mesh.vertex_buffer, gl.STATIC_DRAW);

        const position_size = 3;
        const position_offset = 0;
        gl.vertexAttribPointer(positionAttributeLocation, position_size, gl.FLOAT, false, 0, position_offset);

        const index_buffer = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, index_buffer);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, mesh.index_buffer, gl.STATIC_DRAW);

        gl.bindVertexArray(vao);
        gl.drawElements(gl.TRIANGLES, mesh.index_buffer.length, gl.UNSIGNED_SHORT, 0);
    }
    else {
        const vao = gl.createVertexArray();
        gl.bindVertexArray(vao);

        gl.enableVertexAttribArray(positionAttributeLocation);

        const vertex_buffer = gl.createBuffer();

        gl.bindBuffer(gl.ARRAY_BUFFER, vertex_buffer);
        gl.bufferData(gl.ARRAY_BUFFER, mesh.vertex_buffer, gl.STATIC_DRAW);

        const position_size = 3;
        const position_offset = 0;
        gl.vertexAttribPointer(positionAttributeLocation, position_size, gl.FLOAT, false, 0, position_offset);

        let tri_count = mesh.vertex_buffer.length / 9;
        gl.bindVertexArray(vao);
        gl.drawArrays(gl.TRIANGLES, 0, Math.floor(tri_count));

    }
}

function draw_scene() {
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);


    objects.forEach(object => {
        let current_mesh = meshes.get(object.mesh_id);

        draw_mesh(gl, current_mesh);
    });
}

draw_scene();
